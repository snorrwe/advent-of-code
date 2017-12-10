import os
import re

TEMPLATE = """
build objects(../../$out/{name}/{name}): auto {name}.cpp
build application(../../$out/{name}/{name}): auto objects($out/{name}/*)
"""


def main():
    for root, dirs, files in os.walk('src'):
        for file in files:
            m = re.match(r'(.*day\d+)\.cpp$', file)
            if(m):
                name = m.group(1)
                with open(os.path.join(root, 'build.fox'), 'w') as f:
                    f.write(TEMPLATE.format(name=name))

if __name__ == '__main__':
    main()
