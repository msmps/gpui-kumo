#!/usr/bin/env python3
"""Check native light neutral-off corner pixels; see docs/ring-edge-validation.md.

Requires Pillow in the capture environment (not a Rust build dependency).
Pass the capture and track-origin x/y; default is the 1040x800 gallery position.
"""
import argparse
from PIL import Image

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('capture')
parser.add_argument('--x', type=int, default=57)
parser.add_argument('--y', type=int, default=287)
args = parser.parse_args()
image = Image.open(args.capture).convert('RGB')
# Base track is 36x18. Inspect the four curved 5px corner areas.
xs = list(range(args.x, args.x + 5)) + list(range(args.x + 32, args.x + 37))
ys = list(range(args.y, args.y + 5)) + list(range(args.y + 14, args.y + 19))
minimum = min(min(image.getpixel((x, y))) for x in xs for y in ys)
# This source recipe has pale neutral fill/ring and white thumb. Allow soft
# black thumb shadow, but reject the shader's much darker black RGB fringe.
assert minimum >= 220, f'dark corner fringe: minimum RGB channel {minimum} < 220'
print(f'Light neutral-off corner check passed: minimum channel {minimum}')
