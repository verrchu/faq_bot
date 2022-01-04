#! /usr/bin/env python

import argparse
import os
import sys
import logging
import yaml

NAME_FILE = '_name'
DATA_DIR = '_data'
ROOT = '/'

parser = argparse.ArgumentParser()
parser.add_argument('--dry-run', type=bool, default=False)
parser.add_argument('--path', type=str, required=True)
parser.add_argument('--langs', nargs='+', default=['ru'])
parser.add_argument('--log-level',
                    '-l',
                    type=str,
                    choices=['DEBUG', 'INFO', 'ERROR'],
                    default='INFO')
args = parser.parse_args()

DRY_RUN = args.dry_run
LANGS = set(args.langs)


def process_root(dir):
    logging.info('processing /')
    load_segment_name(dir, base_dir = dir)

    dir_entries = os.listdir(dir)
    dir_entries.remove(NAME_FILE)

    if not len(dir_entries):
        abort(f'{dir} has no child entries')

    if DATA_DIR in dir_entries:
        abort(f'root is not supposed to containe {DATA_DIR}')

    for entry in dir_entries:
        entry = os.path.join(dir, entry)
        if not os.path.isdir(entry):
            abort(f'{entry} is not a directory')

        process_child(entry, base_dir=dir)

def process_child(dir, base_dir):
    root_path = get_root_path(dir, base_dir)
    logging.info(f'processing {root_path}')
    load_segment_name(dir, base_dir)

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

        load_data(data_dir, base_dir)

    for entry in dir_entries:
        entry = os.path.join(dir, entry)
        if not os.path.isdir(entry):
            abort(f'{entry} is not a directory')

        process_child(entry, base_dir=dir)

def load_data(data_dir, base_dir):
    root_path = get_root_path(data_dir.rstrip(DATA_DIR), base_dir)
    logging.info(f'loading data for {root_path}')
    data_entries = os.listdir(data_dir)

    for entry in data_entries:
        entry = os.path.join(data_dir, entry)
        if not os.path.isfile(entry):
            abort(f'data entry {entry} is not a file')

    for lang in LANGS:
        if not lang in data_entries:
            abort(f'{root_path} has no l10n defined for {lang}')

def load_segment_name(dir, base_dir):
    segment = get_last_path_segment(dir, base_dir)
    logging.info(f'loading segment {segment} name')

    dir_entries = os.listdir(dir)
    if not NAME_FILE in dir_entries:
        abort(f'entry {dir} has no {NAME_FILE}')

    name_file = os.path.join(dir, NAME_FILE)
    name_l10n = load_yaml(name_file)

    langs = set(name_l10n.keys())
    for lang in LANGS:
        if not lang in langs:
            abort(f'segment {segment} has no defined l10n for {lang}')
    diff = langs.difference(LANGS)
    if len(diff):
        logging.warning(f'segment {segment} has unused l10n: {diff}')


def get_root_path(dir, base_dir):
    n_bdir = os.path.normpath(base_dir)
    n_dir = os.path.normpath(dir)

    return n_dir.lstrip(n_bdir)

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


if __name__ == '__main__':
    logging.basicConfig(level=args.log_level)
    logging.info(f'loading catalog from {args.path}. dry-run: {DRY_RUN}')
    process_root(args.path)
