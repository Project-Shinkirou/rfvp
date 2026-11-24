# MPEG-1/2 decoder notices

The package includes project code under MPL-2.0 and Rust adaptations of
FFmpeg code under LGPL-2.1-or-later. See `LICENSE-MPL-2.0` and
`LICENSE-LGPL-2.1`.

The FFmpeg-derived material includes the Simple IDCT (`src/video/idct.rs`),
MPEG-1/2 tables and VLC code (`src/video/tables.rs`, `src/video/vlctables.rs`,
`src/video/vlc.rs`), and edge emulation (`src/video/videodsp.rs`). The
corresponding upstream notices are in FFmpeg's `libavcodec` sources:

- `simple_idct_template.c`: Copyright (c) 2001 Michael Niedermayer <michaelni@gmx.at>.
  Based upon some commented-out C code from mpeg2dec (`idct_mmx.c`, written by
  Aaron Holtzman <aholtzma@ess.engr.uvic.ca>).
- `mpeg12.c`, `mpeg12data.c`, and `mpeg12dec.c`: Copyright (c) 2000, 2001
  Fabrice Bellard; Copyright (c) 2002-2004 and 2002-2013 Michael Niedermayer
  <michaelni@gmx.at>.
- `vlc.c`: Copyright (c) 2000, 2001 Fabrice Bellard; Copyright (c) 2002-2004
  Michael Niedermayer <michaelni@gmx.at>; Copyright (c) 2010 Loren Merritt.
- `videodsp_template.c`: Copyright (c) 2002-2012 Michael Niedermayer;
  Copyright (C) 2012 Ronald S. Bultje.

Upstream reference: https://github.com/FFmpeg/FFmpeg/tree/n7.1.1/libavcodec
