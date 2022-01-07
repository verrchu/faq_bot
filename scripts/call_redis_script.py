#! /usr/bin/env python

import logging

from argparse import ArgumentParser
from redis import Redis

parser = ArgumentParser()
parser.add_argument('--path', type=str, required=True)
parser.add_argument('--params', nargs='+', default=[])
parser.add_argument('--db-host', type=str, default='localhost')
parser.add_argument('--db-port', type=int, default=6379)
parser.add_argument('--log-level',
                    '-l',
                    type=str,
                    choices=['DEBUG', 'INFO', 'ERROR'],
                    default='INFO')
args = parser.parse_args()

def call(path, params):
    db = Redis(host=args.db_host, port=args.db_port, decode_responses=True)
    script = open(path, 'r').read()

    print(db.eval(script, 0, *params))

if __name__ == '__main__':
    logging.basicConfig(level=args.log_level)
    logging.info(f'calling redis script {args.path} with args {args.params}')
    call(args.path, args.params)
