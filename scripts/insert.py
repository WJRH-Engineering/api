#!/usr/bin/python

import psycopg2 as postgres
import argparse

parser = argparse.ArgumentParser(description='Add a show to the WJRH schedule')
parser.add_argument('Program')
parser.add_argument('Short Name')
parser.add_argument('DJs')
parser.add_argument('')
