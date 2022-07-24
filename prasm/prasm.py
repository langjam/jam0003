#!/usr/bin/env python3
from odf.opendocument import load
from odf import text, teletype, draw
from odf.element import Element
import os.path
import platform
import shlex
import shutil
import struct
import subprocess
import sys
import zipfile

Page_Attribute = ('urn:oasis:names:tc:opendocument:xmlns:drawing:1.0', 'name')
Class_Attribute = ('urn:oasis:names:tc:opendocument:xmlns:presentation:1.0', 'class')
Href_Attribute = ('http://www.w3.org/1999/xlink', 'href')

# Calculate platform bitness based on size of a pointer
# If result is weird default to 32 bits
Bits = struct.calcsize("P") * 8
if Bits not in [32, 64]:
    Bits = 32

# Match user system as NASM target. Default is Unix-like (supporting elf),
# so dispatch for weird cases
Target = { "Darwin": f"macho", "Windows": f"win" }.get(platform.system(), "elf") + str(Bits)
Exe_Suffix = ".exe" if platform.system() == "Windows" else ".out"

def main(source: str, output: str, assembly: str, nasm: str, object: str, ld: str, no_prelude: bool, prelude: str):
    labels = {}
    mangled_label_to_original = {}
    current_label = ""
    order = []

    def append(line: str):
        if current_label in labels:
            labels[current_label] += "\n" + line
        else:
            labels[current_label] = line

    pres = load(source)

    for page in pres.getElementsByType(text.P):
        if not match_parent_node(page, tag_name='draw:page') or match_parent_node(page, attribute=(Page_Attribute, 'page1')):
            continue

        page_text = teletype.extractText(page)

        if match_parent_node(page, 'text:list-item'):
            # This are regular list nodes
            verbose(f"Adding regular node '{page_text.lower()}' to label '{current_label}'")
            append(page_text.lower())

        if match_parent_node(page, attribute=(Class_Attribute, 'title')):
            # This are titles
            verbose(f"Setting current label to '{current_label}'")
            current_label = mangle_label(page_text)
            if current_label not in order:
                order.append(current_label)
                mangled_label_to_original[current_label] = page_text
            img = find_image(source, page)
            if img is not None:
                append(to_binary(img))

        if match_parent_node(page, 'draw:custom-shape'):
            verbose(f"Appending binary data to label {current_label}")
            if page_text:
                append(to_binary(page_text))
            else:
                append(to_binary('\n'))

    with open(assembly, 'w') as out:
        print("BITS 64\nglobal _start", file=out)

        if not no_prelude:
            with open(prelude, "r") as p:
                shutil.copyfileobj(p, out)

        for label in order:
            if label in labels:
                print(f"{label}:\n{labels[label]}\n{label}$len equ $-{label}", file=out)
            else:
                print(f"[WARNING] Skipping empty label '{mangled_label_to_original[label]}'")

    cmd(nasm, "-f", Target, "-o", object, assembly)
    cmd(ld, "-o", output, object)

def to_binary(s: str | bytes) -> str:
    if isinstance(s, str):
        s = bytes(s, 'utf8')
    return 'db ' + ','.join(map(hex, s))

def find_image(source: str, node: Element) -> bytes | None:
    while node.tagName != "draw:page" and node.parentNode:
        node = node.parentNode

    assert node.tagName == "draw:page", "This should be always true"
    for child in traverse_children(node):
        if child.tagName == "draw:image":
            path = child.attributes[Href_Attribute]
            return read_file_from(source, path)
    return None

def traverse_children(node: Element):
    for child in node.childNodes:
        yield child
        yield from traverse_children(child)

def dump_parents(source: str):
    pres = load(source)
    for page in pres.getElementsByType(text.P):
        if not match_parent_node(page, tag_name='draw:page') or match_parent_node(page, attribute=(Page_Attribute, 'page1')):
            continue

        print(50 * "-")
        print("Parents dump")
        print(50 * "-")

        for p in iterate_parents(page):
            print("Tag: ", p.tagName)
            print("Attr:", p.attributes)
            print("Text:", teletype.extractText(p))
            print()

def read_file_from(presentation: str, sub_path: str) -> bytes:
    with zipfile.ZipFile(presentation) as zip:
        return zip.read(sub_path)

def dump_paths(source: str):
    pres = load(source)
    for page in pres.getElementsByType(text.P):
        if not match_parent_node(page, tag_name='draw:page') or match_parent_node(page, attribute=(Page_Attribute, 'page1')):
            continue
        print("Text:", page)
        print("Path:", get_path_of(page))

def mangle_label(label: str) -> str:
    return label.lower()

def iterate_parents(node: Element):
    while node.parentNode:
        yield node
        node = node.parentNode

def find_parent_node(node: Element, /, tag_name: str | None = None, attribute: tuple | None = None) -> Element | None:
    "Find parent node with appropiate tag name or attribute, depending on which are specified"
    required = int(tag_name is not None) + int(attribute is not None)

    if attribute is not None:
        attr_key, attr_val = attribute
    else:
        # They should be never used but linter would scream at me if i don't include them
        attr_key, attr_val = None, None

    for parent in iterate_parents(node):
        satisfied = int(parent.tagName == tag_name)
        satisfied += int(bool(attribute and parent.attributes.get(attr_key) == attr_val))

        if required == satisfied:
            return node

def match_parent_node(*args, **kwargs) -> bool:
    return find_parent_node(*args, **kwargs) is not None

def get_path_of(node: Element):
    "Returns path of element based on parent nodes"
    return ' / '.join(node.tagName for node in reversed(list(iterate_parents(node))))

def cmd(*command: str, **kwargs) -> subprocess.CompletedProcess:
    if Verbose:
        print("[CMD] %s" % " ".join(map(shlex.quote, command)))

    sys.stdout.flush()
    sys.stderr.flush()
    return subprocess.run(command, **kwargs)

def verbose(*args, **kwargs):
    if Verbose:
        print("[INFO]", *args, **kwargs)

def replace_extension(p: str, ext: str) -> str:
    return f"{os.path.splitext(p)[0]}{ext}"

if __name__ == "__main__":
    from argparse import ArgumentParser
    parser = ArgumentParser(description="Assembler of PRASM assembly language")
    parser.add_argument("source", help="Source file that will be assembled")
    parser.add_argument("args", help="Arguments to pass when running", nargs="*")
    parser.add_argument("--output", "-o", help="Target assembly file", metavar="PATH")
    parser.add_argument("--nasm", help="Path to NASM", metavar="PATH")
    parser.add_argument("--linker", help="Path to linker", dest="ld", metavar="PATH")
    parser.add_argument("--tmpasm", help="Path for intermidiate assembly output", dest="assembly", metavar="PATH")
    parser.add_argument("--tmpobj", help="Path for intermidiate object file", dest="object", metavar="PATH")
    parser.add_argument("-V", "--verbose", help="Print all info", action="store_true")
    parser.add_argument("--dump-parents", help="Dump all parents of text nodes in presentation", action="store_true", dest="dump_parents")
    parser.add_argument("--dump-paths", help="Dump all parents of text nodes in presentation", action="store_true", dest="dump_paths")
    parser.add_argument("--run", help="Run executable after assembling", action="store_true")
    parser.add_argument("--prelude", help="Path to prelude (default the same directory as transpiler", metavar="PATH")
    parser.add_argument("--no-prelude", help="Don't include prelude", action="store_true", dest="no_prelude")
    args = parser.parse_args()

    if args.output   is None: args.output   = replace_extension(args.source, Exe_Suffix)
    if args.assembly is None: args.assembly = replace_extension(args.source, ".nasm")
    if args.object   is None: args.object   = replace_extension(args.source, ".o")
    if args.nasm     is None: args.nasm     = "nasm"
    if args.ld       is None: args.ld       = "ld"
    if args.prelude  is None: args.prelude = os.path.join(os.path.dirname(__file__), "prelude.nasm")
    Verbose = args.verbose

    if args.dump_parents:
        dump_parents(source=args.source)
    elif args.dump_paths:
        dump_paths(source=args.source)
    else:
        main(**dict(k for k in vars(args).items() if k[0] not in ['verbose', 'dump_parents', 'dump_paths', 'run', 'args'] ))
        if args.run:
            if not os.path.isabs(args.output):
                args.output = os.path.join(".", args.output)
            exit(cmd(args.output, *args.args).returncode)
