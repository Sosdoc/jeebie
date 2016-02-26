''' A script for mapping opcode implementation functions to the opcode table '''

import os
import re
import glob

# matches (instruction name)(opcode)(timing)(function name) from comments placed above functions
# if it looks like magic, maybe it is.
FN_REGEX = re.compile(ur'//\s\'(?P<name>.*)\''
                      ur'\s(?P<code>CB [0-9A-F]{2}|[0-9A-F]{2})'
                      ur'\s(?P<time>\d+)'
                      ur'\n*pub fn (?P<fname>.+)\(', re.MULTILINE)

CODES = {}
CB_CODES = {}
MODULES = []

MODULE_IMPORT = u'use gbe::instr::{0}::*;\n'

FILE_HEADER = (u'\nuse gbe::cpu::CPU;\n'
               u'/// The type of functions that implement an opcode.\n'
               u'pub type OpcodeFunc = fn(&mut CPU) -> ();\n')

TABLE_DECL = u'\n\npub static {0}_TABLE : [OpcodeFunc; 256] = [\n'

TABLE_ROW = u'    {:>26}, {:>26}, {:>26}, {:>26},\n'

UNINMPLEMENTED = u'panic!("Not implemented.")'


def process_opcode_file(file_name):
    ''' reads a file with opcode implementations '''

    source_text = open(file_name, 'r').read()

    for match in FN_REGEX.finditer(source_text):
        group = match.groupdict()

        if group['code'].startswith('CB'):
            code = int(group['code'][-2:], 16)
            CB_CODES[code] = {
                'name': group['name'],
                'fn': group['fname'],
                'timing': group['time']
            }
        else:
            code = int(group['code'], 16)
            CODES[code] = {
                'name': group['name'],
                'fn': group['fname'],
                'timing': group['time']
            }

    module_name = os.path.splitext(os.path.basename(file_name))[0]

    if module_name != 'mod':
        MODULES.append(module_name)


def build_output():
    output = []

    for module in MODULES:
        output.append(MODULE_IMPORT.format(module))

    output.append(FILE_HEADER)

    output.append(TABLE_DECL.format('OPCODE'))

    for i in xrange(0, 256, 4):
        if i % 16 == 0:
            output.append('    // ' + format(i, '#04x') + '\n')

        row = TABLE_ROW.format(func(i), func(i + 1), func(i + 2), func(i + 3))
        output.append(row)

    output.append('];')

    output.append(TABLE_DECL.format('CB_OPCODE'))

    for i in xrange(0, 256, 4):
        if i % 16 == 0:
            output.append('    // ' + format(i, '#04x') + '\n')

        row = TABLE_ROW.format(func_cb(i), func_cb(
            i + 1), func_cb(i + 2), func_cb(i + 3))
        output.append(row)

    output.append('];')

    return output


def func(opcode):
    return CODES[opcode]['fn'] if opcode in CODES else UNINMPLEMENTED


def func_cb(opcode):
    return CB_CODES[opcode]['fn'] if opcode in CB_CODES else UNINMPLEMENTED

if __name__ == "__main__":

    for src_file in glob.glob('src/gbe/instr/*.rs'):
        process_opcode_file(src_file)

    with open('src/gbe/opcodes.rs', 'w') as out_file:
        out_file.writelines(build_output())
