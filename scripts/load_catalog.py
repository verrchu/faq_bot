#! /usr/bin/env python

import os
import sys
import logging
import yaml

from argparse import ArgumentParser
from hashlib import md5
from redis import Redis
from time import time

NAME_FILE = '_name'
DATA_DIR = '_data'
ROOT = '/'

parser = ArgumentParser()
parser.add_argument('--dry-run', action='store_true')
parser.add_argument('--path', type=str, required=True)
parser.add_argument('--langs', nargs='+', default=['ru'])
parser.add_argument('--db-host', type=str, default='localhost')
parser.add_argument('--db-port', type=int, default=6379)
parser.add_argument('--log-level',
                    '-l',
                    type=str,
                    choices=['DEBUG', 'INFO', 'ERROR'],
                    default='INFO')
args = parser.parse_args()

DRY_RUN = args.dry_run
LANGS = set(args.langs)

db = Redis(host=args.db_host, port=args.db_port, decode_responses=True)


def process_root(dir):
    logging.info('processing /')
    load_segment_name(dir, base_dir=dir)

    path_hash = hash(ROOT)
    logging.debug(f'inserting {ROOT} as {path_hash}')
    DRY_RUN or db.hset('key_hashes', path_hash, ROOT)  # set hash -> path

    dir_entries = os.listdir(dir)
    dir_entries.remove(NAME_FILE)

    if not len(dir_entries):
        abort(f'{dir} has no child entries')

    if DATA_DIR in dir_entries:
        abort(f'root is not supposed to containe {DATA_DIR}')

    for entry in dir_entries:
        logging.debug(f'adding {entry} to {ROOT}:next')

        segment = entry
        key = os.path.join(ROOT, entry)

        DRY_RUN or db.sadd(f'{ROOT}:next', key)

        entry = os.path.join(dir, entry)
        if not os.path.isdir(entry):
            abort(f'{entry} is not a directory')

        process_child(entry, base_dir=dir)

        if not DRY_RUN:
            for lang in LANGS:
                segment_name = db.get(f'{segment}:name:{lang}')
                key_icon = db.get(f'{key}:icon')

                l10n = f'{key_icon} {segment_name}' if key_icon else segment_name

                db.hset(f'/:next:{lang}', key, l10n)



def process_child(dir, base_dir):
    root_path = get_root_path(dir, base_dir)
    logging.info(f'processing {root_path}')
    load_segment_name(dir, base_dir)
    load_key_icon(dir, base_dir)

    path_hash = hash(root_path)
    logging.debug(f'inserting {root_path} as {path_hash}')
    DRY_RUN or db.hset('key_hashes', path_hash, root_path)  # set hash -> path

    dir_entries = os.listdir(dir)
    dir_entries.remove(NAME_FILE)

    if not len(dir_entries):
        abort(f'{dir} has no child entries')

    if DATA_DIR in dir_entries:
        data_dir = os.path.join(dir, DATA_DIR)
        dir_entries.remove(DATA_DIR)
        if not os.path.isdir(data_dir):
            abort(f'{data_dir} is not a direactory')
        if len(dir_entries) > 1:
            abort(f'{data_dir} has unexpected neighbours: {dir_entries}')

        logging.debug(f'adding {root_path} to data_entries')
        DRY_RUN or db.sadd('data_entries', root_path)

        load_data(data_dir, base_dir)

    for entry in dir_entries:
        logging.debug(f'adding {entry} to {root_path}:next')

        segment = entry
        key = os.path.join(root_path, entry)

        DRY_RUN or db.sadd(f'{root_path}:next', key)

        entry = os.path.join(dir, entry)
        if not os.path.isdir(entry):
            abort(f'{entry} is not a directory')

        process_child(entry, base_dir)

        if not DRY_RUN:
            for lang in LANGS:
                segment_name = db.get(f'{segment}:name:{lang}')
                key_icon = db.get(f'{key}:icon')

                l10n = f'{key_icon} {segment_name}' if key_icon else segment_name

                db.hset(f'{root_path}:next:{lang}', key, l10n)


def load_data(data_dir, base_dir):
    root_path = get_root_path(data_dir.removesuffix(DATA_DIR), base_dir)
    logging.info(f'loading data for {root_path}')
    data_entries = set(os.listdir(data_dir))

    DRY_RUN or db.setnx(f'{root_path}:created', unixtime())

    for entry in data_entries:
        entry = os.path.join(data_dir, entry)
        if not os.path.isfile(entry):
            abort(f'data entry {entry} is not a file')

    for lang in LANGS:
        if not lang in data_entries:
            abort(f'{root_path} has no l10n defined for {lang}')
        data = open(os.path.join(data_dir, lang), 'r').read()
        logging.debug(f'loading {root_path} data for {lang}')
        DRY_RUN or db.set(f'{root_path}:data:{lang}', data)

    diff = data_entries.difference(LANGS)
    if len(diff):
        logging.warning(f'{root_path} has unused data entries: {diff}')


def load_key_icon(dir, base_dir):
    root_path = get_root_path(dir, base_dir)

    dir_entries = os.listdir(dir)
    if not NAME_FILE in dir_entries:
        abort(f'entry {dir} has no {NAME_FILE}')

    name_file = os.path.join(dir, NAME_FILE)
    name_file = load_yaml(name_file)

    if not name_file:
        abort(f'entry {dir} has invalid {NAME_FILE}')

    if 'icon' in name_file:
        icon = name_file['icon']
        logging.info(f'loading icon {icon} for {root_path}')
        DRY_RUN or db.set(f'{root_path}:icon', icon)


def load_segment_name(dir, base_dir):
    segment = get_last_path_segment(dir, base_dir)
    logging.info(f'loading segment {segment} name')

    dir_entries = os.listdir(dir)
    if not NAME_FILE in dir_entries:
        abort(f'entry {dir} has no {NAME_FILE}')

    name_file = os.path.join(dir, NAME_FILE)
    name_file = load_yaml(name_file)

    if not name_file:
        abort(f'entry {dir} has invalid {NAME_FILE}')

    name_l10n = name_file['l10n']

    langs = set(name_l10n.keys())
    for lang in LANGS:
        if not lang in langs:
            abort(f'segment {segment} has no defined l10n for {lang}')
        logging.debug(
            f'loading segment {segment} l10n for {lang}: {name_l10n[lang]}')
        DRY_RUN or db.set(f'{segment}:name:{lang}', name_l10n[lang])

    diff = langs.difference(LANGS)
    if len(diff):
        logging.warning(f'segment {segment} has unused l10n: {diff}')


def get_root_path(dir, base_dir):
    n_bdir = os.path.normpath(base_dir)
    n_dir = os.path.normpath(dir)

    diff = n_dir.removeprefix(n_bdir)
    return diff if diff else '/'


def get_last_path_segment(dir, base_dir):
    if not get_root_path(dir, base_dir) == ROOT:
        return os.path.basename(os.path.normpath(dir))

    return ROOT


def load_yaml(file):
    logging.debug(f'reading yaml from {file}')
    with open(file, 'r') as stream:
        try:
            return yaml.safe_load(stream)
        except yaml.YAMLError as err:
            abort(f'failed to load yaml from {file}: {err}')


def abort(reason):
    logging.critical(reason)
    sys.exit(1)


def hash(input):
    return md5(input.encode('utf-8')).hexdigest()


def unixtime():
    return int(time())


if __name__ == '__main__':
    logging.basicConfig(level=args.log_level)
    logging.info(f'loading catalog from {args.path}. dry-run: {DRY_RUN}')
    process_root(args.path)
