# WMV/WMA decoder notices

The package includes project code under MPL-2.0 and Rust adaptations of
FFmpeg code under LGPL-2.1-or-later. See `LICENSE-MPL-2.0` and
`LICENSE-LGPL-2.1`. The SPDX notices in `src/wma/pro.rs` and
`src/wma/pro_tables.rs` apply to those modules; the FFmpeg-derived material
also includes portions of the ASF, WMV2/MSMPEG4, VC-1 and WMA v1/v2 code.

Corresponding upstream notices (paths relative to FFmpeg):

- `libavformat/asfdec_f.c`: Copyright (c) 2000, 2001 Fabrice Bellard.
- `libavcodec/simple_idct.c` and `simple_idct_template.c`: Copyright (c) 2001
  Michael Niedermayer <michaelni@gmx.at>. Based upon some commented-out C code
  from mpeg2dec (`idct_mmx.c`, written by Aaron Holtzman
  <aholtzma@ess.engr.uvic.ca>).
- `libavcodec/msmpeg4dec.c`, `msmpeg4data.c` and `msmpeg4_vc1_data.c`:
  Copyright (c) 2001 Fabrice Bellard; Copyright (c) 2002-2013 and 2002-2004
  Michael Niedermayer <michaelni@gmx.at>.
- `libavcodec/mpegvideo.c`: Copyright (c) 2000, 2001 Fabrice Bellard;
  Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>.
- `libavcodec/wmv2dsp.c`: the WMV2 IDCT in `src/na_wmv2dsp.rs` is a Rust
  adaptation of this LGPL-2.1-or-later source.
- `libavcodec/wmv2.h`, `wmv2dec.c`, `wmadata.h` and `wmadec.c`:
  Copyright (c) 2002 The FFmpeg Project.
- `libavcodec/wma.c`: Copyright (c) 2002-2007 The FFmpeg Project.
- `libavcodec/vc1.c`, `vc1_mc.c`, `vc1_loopfilter.c`, `vc1dsp.c`,
  `vc1data.c`, `vc1data.h` and `vc1_vlc_data.h`: Copyright (c) 2011 Mashiat
  Sarker Shakkhar; Copyright (c) 2006 and 2006-2007 Konstantin Shishkov.
- `libavcodec/wmaprodec.c` and `wmaprodata.h`: Copyright (c) 2007 Baptiste
  Coudurier, Benjamin Larsson, Ulion; Copyright (c) 2008-2011 Sascha Sommer,
  Benjamin Larsson (`wmaprodec.c`); Copyright (c) 2008-2009 Sascha Sommer
  (`wmaprodata.h`).
- `libavcodec/get_bits.h`: Copyright (c) 2004 Michael Niedermayer <michaelni@gmx.at>.
- `libavcodec/vlc.c`: Copyright (c) 2000, 2001 Fabrice Bellard; Copyright (c)
  2002-2004 Michael Niedermayer <michaelni@gmx.at>; Copyright (c) 2010 Loren Merritt.

Upstream reference: https://github.com/FFmpeg/FFmpeg/tree/n7.1.1
