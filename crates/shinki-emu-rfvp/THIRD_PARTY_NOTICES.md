# 第三方来源与许可（shinki-emu-rfvp）

## RFVP 覆盖源码

本仓库为 [`xmoezzz/rfvp`](https://github.com/xmoezzz/rfvp) 的完整 fork，发布分支为 [`Project-Shinkirou/rfvp@shinki`](https://github.com/Project-Shinkirou/rfvp/tree/shinki)。通用基线为 `3359d8fbea3c53d6d262a9f71b6d4d79888c2368`，上游历史基线为 `af557db750c7833274f22a6d82b0dc33cdea2f85`，crate 版本 0.6.0。变更分类见 [MODIFICATIONS.md](MODIFICATIONS.md)。

`shinki-latest` 在上述历史上合并稳定发布 `0.7.0`（`99435f22e94eaec27c0c1b9f22cc2a6b1910d7d1`），保留覆盖源码及既有许可。

完整 MPL-2.0 许可位于仓库根 [`LICENSE`](../../LICENSE)。发行 `shinki_emu_rfvp.dll` 时须保留与二进制对应的覆盖源码（或有效的 source offer）、该许可与本目录的变更记录，不以旧已发布基线替代当前二进制实际使用的源码。

适配包只提供 C ABI v3 注册、原生 owner、CPU lease 与渲染尺寸换算，复用原 App、VM、GPU、视频、设备音频和 V1/RFVS 保存，没有 Host advance 或 PCM sink。

## ShinkiEMU 与 RoukakuEngine 组件

`shinki-emu-abi`（C ABI v3）与 host-side `shinki-emu-build` 来自 [`Project-Shinkirou/ShinkiEMU@cb8d9dd22d7d0ab129085b324c94d6dc81fa19e3`](https://github.com/Project-Shinkirou/ShinkiEMU/commit/cb8d9dd22d7d0ab129085b324c94d6dc81fa19e3)，采用 MIT OR Apache-2.0。`roukaku-subtitle` 来自 [`Project-Shinkirou/RoukakuEngine@8392cf161800f7684af7704eb95f7a5526d2015d`](https://github.com/Project-Shinkirou/RoukakuEngine/commit/8392cf161800f7684af7704eb95f7a5526d2015d)。manifest 与 `Cargo.lock` 固定这些 git revision。

## ASS 字幕依赖

`roukaku-subtitle` 使用 [libass](https://github.com/libass/libass)（ISC），原生依赖按 overlay triplet `x64-windows-static-md-dynbidi`（RoukakuEngine `Engine/Source/Runtime/roukaku-subtitle/triplets/`）安装：libass、FreeType（选择 FreeType License）、HarfBuzz（Old MIT）、Brotli（MIT）、bzip2、libpng、zlib 静态链接进核心，FriBidi（LGPL-2.1）以 `fribidi-0.dll` 动态链接并随核心目录分发，用户可替换为自行构建的兼容 DLL。发行时须逐字保留实际 vcpkg 安装 `share/<port>/copyright` 的全部条款，附 LGPL-2.1 文本及 FriBidi 对应源码；静态依赖即使没有单独 DLL 也须保留条款。本产品使用 FreeType Project（<https://freetype.org/>）的软件。字幕字体由游戏侧显式资源提供，须保留各自许可。

## Kira 与 CPAL

Kira 0.10.8 来自 [`Project-Shinkirou/kira@d2f10db15d9c451c7b2fd33043ae7732e75bee5d`](https://github.com/Project-Shinkirou/kira/commit/d2f10db15d9c451c7b2fd33043ae7732e75bee5d)，CPAL 0.15.3 经 workspace `[patch.crates-io]` 来自 `Project-Shinkirou/cpal@65ac8a6ac1a635cc26bfc5af331295b052044908`，均按各自上游许可发布，须保留其许可文本。

## 系统字体

原生 App 路径沿用上游 `FontEnumerator::new`，编译期嵌入 `MSGOTHIC.TTF`、`MSMINCHO.TTF`、`MS-PGothic.ttf`、`MS-PMincho-2.ttf`。这四个字体文件由上游 [`xmoezzz/rfvp@f60d8660`](https://github.com/xmoezzz/rfvp/commit/f60d8660d800fe16a4da485ef50e010e7b1ec2e5) 加入，存在于上游基线 `af557db750c7833274f22a6d82b0dc33cdea2f85`，核心二进制按上游原样包含并发布。

## WMV 解码来源

`crates/na_wmv_player` 中的 WMA Pro 解码器 `src/wma/pro.rs` 和码表
`src/wma/pro_tables.rs` 均保留 `SPDX-License-Identifier: LGPL-2.1-or-later`
文件头，分别对应 FFmpeg 的 `libavcodec/wmaprodec.c` 和
`libavcodec/wmaprodata.h`。WMV2、VC-1、ASF 和 WMA v1/v2 中也有对照确认的
FFmpeg 移植代码，来源及著作权声明见 [WMV/WMA 许可说明](../na_wmv_player/NOTICE.md)。

`wmv-decoder` 的 Cargo 清单用 `MPL-2.0 AND LGPL-2.1-or-later` 记录包内既有条款：
RFVP/Siglus 项目代码保留 MPL-2.0，来自 FFmpeg 的部分保留 LGPL-2.1-or-later。
两份许可文本随 crate 提供。发布前仍须按实际链接产物核对并落实对应源码及重新链接义务。
