#!/usr/bin/env python3

import json
import sys
import pprint

if __name__ == '__main__':
	langs = json.load(sys.stdin)

	# Correct some entries which don't make sense for our use case.
	langs['XML'] = ['.xml']
	langs['Motorola 68K Assembly'] = ['.x68']
	langs['RenderScript'] = ['.rsh']
	del langs['Objective-C']
	del langs['Vim Help File']
	langs['C++'] = [x for x in langs['C++'] if x != '.h']

	i = 0
	out_names = []
	out_exts = []
	for name, exts in langs.items():
		out_names.append(name)
		for ext in exts:
			out_exts.append((ext[1:], i))
		i += 1
	out_exts = str(out_exts).replace('\'', '"')

	print('use std::collections::HashMap;')
	print(f'pub static LANG_NAMES: &[&str; {i}] = &{json.dumps(out_names)};')
	print('pub fn get_language_index() -> HashMap<&\'static str, usize> {')
	print('	HashMap::from(')
	print(f'	{out_exts}')
	print('	)')
	print('}')
