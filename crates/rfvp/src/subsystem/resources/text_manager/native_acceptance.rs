use super::*;
use crate::script::{context::Context, parser::{Nls, Parser}, Variant};
use crate::subsystem::snapshot_codec;

#[test]
fn native_literal_identity_read_flags_and_saved_continuation_roundtrip() {
    // Original site -> ordinary PushString island -> original continuation.
    let mut bytes=vec![0;24];
    bytes[4..9].copy_from_slice(&[6,12,0,0,0]);
    bytes[9]=4;
    bytes[12..19].copy_from_slice(&[14,5,0xd6,0xd0,0xce,0xc4,0]);
    bytes[19..24].copy_from_slice(&[6,9,0,0,0]);
    bytes[..4].copy_from_slice(&24u32.to_le_bytes());
    bytes.extend_from_slice(&4u32.to_le_bytes());
    bytes.extend_from_slice(&[0,0,0,0,0,0,1,0,0,0,0,0]);
    let mut parser=Parser::from_bytes(bytes,Nls::GBK).unwrap();
    let mut context=Context::new(4,0);
    let before=context.capture_snapshot_v1();
    context.jmp(&mut parser).unwrap();
    assert_eq!(context.capture_snapshot_v1().cur_stack_pos,before.cur_stack_pos);
    context.push_string(&mut parser).unwrap();
    let snapshot=context.capture_snapshot_v1();
    let encoded=snapshot_codec::encode(&snapshot).unwrap();
    let mut restored=Context::new(0,0);
    restored.apply_snapshot_v1(&snapshot_codec::decode(&encoded).unwrap());
    restored.jmp(&mut parser).unwrap();
    assert_eq!(restored.get_pc(),9);
    let value=restored.capture_snapshot_v1();
    let Variant::ConstString(text,identity)=&value.stack[value.cur_stack_base+value.cur_stack_pos-1] else {panic!("lost native string type")};
    assert_eq!(text,"中文");assert_eq!(*identity,14);
    let mut manager=TextManager::new();
    assert!(manager.mark_readed_text_first(*identity));
    assert!(!manager.mark_readed_text_first(*identity));
    let saved=snapshot_codec::encode(&manager.capture_snapshot_v1()).unwrap();
    let mut restored=TextManager::new();
    restored.apply_snapshot_v1(&snapshot_codec::decode(&saved).unwrap());
    assert!(!restored.mark_readed_text_first(*identity));
    assert!(restored.mark_readed_text_first(6)); // Original external-table identity is distinct.
}

#[test]
fn native_island_inside_call_preserves_saved_frame_and_return_value() {
    let mut bytes=vec![0;36];
    bytes[..4].copy_from_slice(&36u32.to_le_bytes());
    bytes[4..9].copy_from_slice(&[2,12,0,0,0]);
    bytes[9]=4;
    bytes[12..20].copy_from_slice(&[1,0,0,6,24,0,0,0]);
    bytes[20]=5;
    bytes[24..31].copy_from_slice(&[14,5,0xd6,0xd0,0xce,0xc4,0]);
    bytes[31..36].copy_from_slice(&[6,20,0,0,0]);
    bytes.extend_from_slice(&4u32.to_le_bytes());
    bytes.extend_from_slice(&[0,0,0,0,0,0,1,0,0,0,0,0]);
    let mut parser=Parser::from_bytes(bytes,Nls::GBK).unwrap();
    let mut context=Context::new(4,0);
    context.call(&mut parser).unwrap();
    context.init_stack(&mut parser).unwrap();
    let frame=context.capture_snapshot_v1();
    context.jmp(&mut parser).unwrap();
    assert_eq!(context.capture_snapshot_v1().cur_stack_base,frame.cur_stack_base);
    context.push_string(&mut parser).unwrap();
    let saved=snapshot_codec::encode(&context.capture_snapshot_v1()).unwrap();
    let mut restored=Context::new(0,0);
    restored.apply_snapshot_v1(&snapshot_codec::decode(&saved).unwrap());
    restored.jmp(&mut parser).unwrap();
    restored.retv().unwrap();
    assert_eq!(restored.get_pc(),9);
    assert!(matches!(&restored.capture_snapshot_v1().return_value,Variant::ConstString(text,26) if text=="中文"));
}

#[test]
#[ignore = "requires explicitly authorized private converted output and reference; never launches a game"]
fn private_native_conversion_and_selected_glyphs() {
    use sha2::{Digest,Sha256};
    let reference=PathBuf::from(std::env::var_os("RFVP_PRIVATE_REFERENCE").expect("RFVP_PRIVATE_REFERENCE"));
    let output=PathBuf::from(std::env::var_os("RFVP_PRIVATE_NATIVE").expect("RFVP_PRIVATE_NATIVE"));
    let old=fs::read(reference.join("Sakura.hcb")).unwrap();
    let native=fs::read(output.join("Sakura.hcb")).unwrap();
    let table=fs::read(reference.join("shinki-translations.bin")).unwrap();
    assert_eq!(&table[..8],b"AFVPTR1\0");
    assert!(!output.join("shinki-translations.bin").exists());
    let mut rows=std::collections::BTreeMap::new();
    let count=u32::from_le_bytes(table[72..76].try_into().unwrap()) as usize;
    let mut at=76;
    for _ in 0..count {
        let key=u32::from_le_bytes(table[at..at+4].try_into().unwrap()) as usize;
        let n=u32::from_le_bytes(table[at+4..at+8].try_into().unwrap()) as usize;at+=8;
        rows.insert(key,std::str::from_utf8(&table[at..at+n]).unwrap().to_owned());at+=n;
    }
    assert_eq!(at,table.len());
    let old_parser=Parser::from_bytes(old.clone(),Nls::ShiftJIS).unwrap();
    let mut parser=Parser::from_bytes(native.clone(),Nls::GBK).unwrap();
    let widths=[1,3,5,3,1,1,5,5,1,1,5,3,2,5,0,3,2,3,2,1,1,3,2,3,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let mut pc=4;let mut chars=std::collections::BTreeSet::new();let mut strings=0;let mut islands=0;let mut edges=0;let mut ruby=0;let mut long=0;
    let mut examples:[Option<String>;3]=std::array::from_fn(|_|None);
    while pc<old_parser.sys_desc_offset as usize {
        let op=old[pc];
        let size=if op==14 {2+old[pc+1] as usize} else {widths[op as usize]};
        if op==14 {
            let expected=rows.get(&(pc+2)).cloned().unwrap_or_else(||old_parser.read_cstring(pc+2,old[pc+1] as usize).unwrap());
            let mut context=Context::new(pc as u32,0);
            if native[pc]==6 {context.jmp(&mut parser).unwrap();islands+=1;}
            context.push_string(&mut parser).unwrap();
            let next=context.get_pc();
            let value=context.capture_snapshot_v1();
            let Variant::ConstString(actual,id)=&value.stack[value.cur_stack_base+value.cur_stack_pos-1] else {panic!("not a native literal")};
            assert_eq!(actual,&expected,"native literal mismatch at {pc}");
            assert!(*id<0x800000);
            if native[pc]==6 {context.jmp(&mut parser).unwrap();assert_eq!(context.get_pc(),pc+size);}
            else {assert!(native[next..pc+size].iter().all(|b|*b==0));}
            if rows.contains_key(&(pc+2)) && actual.chars().any(|c|('\u{4e00}'..='\u{9fff}').contains(&c)) {
                let categories=[actual.chars().count()<=12 && !actual.contains('/'),actual.contains('[') && actual.contains('|'),actual.len()>254];
                for i in 0..3 {if categories[i] && examples[i].is_none() {examples[i]=Some(actual.clone());}}
            }
            chars.extend(actual.chars().filter(|ch|!ch.is_whitespace()));
            ruby+=usize::from(actual.contains('|') && actual.contains('['));
            long+=usize::from(actual.len()>254);strings+=1;
        } else if matches!(op,2|6|7) {
            let target=u32::from_le_bytes(old[pc+1..pc+5].try_into().unwrap());
            if target<old_parser.sys_desc_offset {assert_eq!(&native[pc..pc+5],&old[pc..pc+5]);edges+=1;}
        }
        pc+=size;
    }
    assert_eq!(strings,61164);assert_eq!(rows.len(),57053);assert!(ruby>0 && long>0 && islands>0);
    use crate::host_api::{RfvpStorage,std_fs::StdDir};
    use crate::subsystem::resources::vfs::{Vfs,VfsFile,HostPackFile};
    use std::io::Read;
    let source_disk=StdDir::new(&reference);
    let native_disk=std::sync::Arc::new(StdDir::new(&output));
    let native_vfs=Vfs::from_dir(Nls::GBK,native_disk.clone()).unwrap();
    let mut resource_hits=0;let mut payload_bytes=0u64;
    let mut left=vec![0u8;256*1024];let mut right=vec![0u8;256*1024];
    for filename in ["bgm.bin","graph.bin","graph_bg.bin","graph_bs.bin","graph_sd.bin",
        "graph_vis.bin","graph_vish.bin","se.bin","se_env.bin","se_sys.bin","voice.bin"] {
        let folder=filename.strip_suffix(".bin").unwrap();
        let original_pack=VfsFile::from_pack_source(folder.into(),std::sync::Arc::new(HostPackFile::new(source_disk.open(filename).unwrap()).unwrap()),Nls::ShiftJIS).unwrap();
        let native_pack=VfsFile::from_pack_source(folder.into(),std::sync::Arc::new(HostPackFile::new(native_disk.open(filename).unwrap()).unwrap()),Nls::GBK).unwrap();
        assert_eq!(original_pack.entries.len(),native_pack.entries.len());
        for (name,old_entry) in &original_pack.entries {
            let entry=native_pack.entries.get(name).expect("native resource name lookup lost");
            assert_eq!((entry.offset,entry.size),(old_entry.offset,old_entry.size));
            let mut original=original_pack.open_stream(name).unwrap();
            let mut converted=native_pack.open_stream(name).unwrap();
            let mut remaining=entry.size;
            while remaining>0 {
                let n=remaining.min(left.len() as u64) as usize;
                original.read_exact(&mut left[..n]).unwrap();converted.read_exact(&mut right[..n]).unwrap();
                assert_eq!(&left[..n],&right[..n],"native archive payload changed");
                remaining-=n as u64;payload_bytes+=n as u64;
            }
            let path=format!("{folder}/{name}");
            let (mut actual,len)=native_vfs.open_stream_with_len(&path).expect("ordinary VFS resource reference misses");
            let expected_len=reference.join(&path).metadata().map_or(entry.size,|m|m.len());
            assert_eq!(len,Some(expected_len));
            let n=expected_len.min(16) as usize;actual.read_exact(&mut right[..n]).unwrap();
            resource_hits+=1;
        }
    }
    println!("ordinary native VFS: resource_hits={resource_hits}, exact_payload_bytes={payload_bytes}, preserved offsets/sizes/Unicode names");
    let old_global=crate::subsystem::global_savedata::try_decode_global_savedata_v1(&fs::read(reference.join("save/rfvp_global.bin")).unwrap()).unwrap().unwrap();
    println!("reference native saved font: family={:?}, system_id={}",old_global.current_font_name,old_global.system_fontface_id);
    let mut fonts=FontEnumerator::new();
    let id=fonts.add_font_file(&output.join("font/HanaMinA.ttf")).unwrap();
    let selected_family=fonts.get_font_name(id).unwrap();
    fonts.set_forced_font(Some(id));
    // Restored script/global face requests must not undo the configured primary.
    fonts.set_current_font_name(&old_global.current_font_name);
    fonts.set_system_fontface_id(old_global.system_fontface_id);
    let mut rendered_slots=0;
    for entry in fs::read_dir(reference.join("save")).unwrap() {
        let path=entry.unwrap().path();
        if !path.file_name().unwrap().to_string_lossy().starts_with("rfvp_s") {continue;}
        let snapshot=crate::subsystem::save_state::try_decode_state_chunk_v1(&fs::read(&path).unwrap()).unwrap().unwrap();
        let mut gaiji=GaijiManager::new();
        gaiji.apply_snapshot_v1(&snapshot.motion.gaiji_manager,&native_vfs).unwrap();
        for (slot, saved) in snapshot.motion.text_manager.items.iter().enumerate() {
            if !saved.loaded || saved.content_text.is_empty() || saved.w==0 || saved.h==0 {continue;}
            let mut item=TextItem::new();
            item.apply_snapshot_v1(saved);
            item.rasterize_full(&fonts,&gaiji).unwrap();
            // Instant text owns a tight pixel_buffer; revealing text retains the
            // complete layout in full_buffer. Match the native cache contract.
            let (w,h,pixels)=if item.speed==0 {
                (item.surface_width_px,item.surface_height_px,&item.pixel_buffer)
            } else {
                let (w,h)=item.raster_dimensions();(w,h,&item.full_buffer)
            };
            assert!(pixels.chunks_exact(4).any(|pixel|pixel[3]>0),
                "empty actual slot {slot}: chars={}, nonspace={}, tokens={}, offset={},{} size={}x{} font={} color={:?} ruby_color={:?}",
                saved.content_text.chars().count(),saved.content_text.chars().filter(|c|!c.is_whitespace()).count(),
                item.content_items.len(),saved.offset_x,saved.offset_y,saved.w,saved.h,saved.text_size1,saved.color1,saved.color2);
            let image=image::RgbaImage::from_raw(w,h,pixels.clone()).unwrap();
            let proof_dir=PathBuf::from(std::env::var_os("RFVP_PRIVATE_PROOF").unwrap()).parent().unwrap().to_owned();
            image.save(proof_dir.join(format!("saved-text-{}-{slot}.png",path.file_stem().unwrap().to_string_lossy()))).unwrap();
            println!("actual saved text renderer: slot={slot}, main_face_id={}, ruby_face_id={}, main_size={}, ruby_size={}, selected_family={selected_family}, raster={w}x{h}",saved.text_font_idx1,saved.text_font_idx2,saved.text_size1,saved.text_size2);
            rendered_slots+=1;
        }
    }
    assert!(rendered_slots>0);
    let proof_dir=PathBuf::from(std::env::var_os("RFVP_PRIVATE_PROOF").unwrap()).parent().unwrap().to_owned();
    for (kind,text) in ["short","ruby","long"].into_iter().zip(examples) {
        let text=text.expect("missing real native Chinese category");
        let mut text_manager=TextManager::new();
        text_manager.set_text_buff(0,1280,256);
        text_manager.set_text_font_idx1(0,FONTFACE_CURRENT);
        text_manager.set_text_font_idx2(0,FONTFACE_MS_GOTHIC);
        text_manager.set_text_size1(0,26);
        text_manager.set_text_size2(0,12);
        text_manager.set_text_color1(0,&ColorItem::white());
        text_manager.set_text_speed(0,0);
        text_manager.set_text_ruby_mode(0,2);
        text_manager.set_text_content(0,&text);
        if kind=="ruby" {assert!(text_manager.items[0].content_items.iter().any(|item|matches!(item,FontItem::RubyFont(..))));}
        let Some(TextSlotSurfaceUpdate::Pixels(surface))=text_manager.rasterize_slot_if_needed(0,&fonts,&GaijiManager::new(),true).unwrap() else {panic!("native Chinese surface is empty")};
        let pixels=text_manager.slot_rgba_bytes(0).unwrap();
        assert!(pixels.chunks_exact(4).any(|pixel|pixel[3]>0));
        image::RgbaImage::from_raw(surface.width,surface.height,pixels.to_vec()).unwrap().save(proof_dir.join(format!("native-Chinese-{kind}.png"))).unwrap();
    }
    println!("ordinary native TextManager: real Chinese short/ruby/long literals rendered with configured primary and native ruby tokenization");
    let mut samples=image::GrayImage::new(1024,((chars.len()+31)/32*40) as u32);
    for style in [FONTFACE_CURRENT,FONTFACE_MS_GOTHIC,FONTFACE_MS_MINCHO,FONTFACE_MS_PGOTHIC,FONTFACE_MS_PMINCHO] {
        let set=fonts.get_font_fallback_set(style);
        for (i,ch) in chars.iter().copied().enumerate() {
            assert!(set.primary().has_glyph(ch),"selected native face lacks U+{:04X}",ch as u32);
            assert!(std::ptr::eq(set.for_char(ch),set.primary()));
            for px in [12.0,24.0] {
                let (metrics,pixels)=set.for_char(ch).rasterize(ch,px);
                if ch.is_alphanumeric() {assert!(pixels.iter().any(|p|*p>0),"empty native glyph U+{:04X}",ch as u32);}
                if style==FONTFACE_CURRENT && px==24.0 {
                    for y in 0..metrics.height.min(40) {for x in 0..metrics.width.min(32) {
                        samples.put_pixel((i%32*32+x) as u32,(i/32*40+y) as u32,image::Luma([pixels[y*metrics.width+x]]));
                    }}
                }
            }
        }
    }
    let proof=PathBuf::from(std::env::var_os("RFVP_PRIVATE_PROOF").expect("RFVP_PRIVATE_PROOF"));
    samples.save(proof).unwrap();
    println!("native acceptance: strings={strings}, translated={}, islands={islands}, unchanged_edges={edges}, ruby_literals={ruby}, long_utf8_literals={long}, selected_face=HanaMinA, unique_chars={}, missing=0, output_sha256={:x}",rows.len(),chars.len(),Sha256::digest(&native));
}
