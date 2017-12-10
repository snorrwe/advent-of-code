import os
import re

FOX_TEMPLATE = """
fixed_out = ../../$out
build objects($fixed_out/{name}/{name}): auto {name}.cpp
build application($fixed_out/{name}/{name}): auto objects($fixed_out/{name}/{name})
"""


def create_fox(file):
    m = re.match(r'(.*day\d+)\.cpp$', file)
    if(m):
        name = m.group(1)
        with open(os.path.join(root, 'build.fox'), 'w') as f:
            f.write(FOX_TEMPLATE.format(name=name))


def main():
    for root, dirs, files in os.walk('src'):
        for file in files:
            create_fox(file)

if __name__ == '__main__':
    main()
