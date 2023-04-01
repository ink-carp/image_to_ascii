pub mod func{
    use std::io::Read;
    use std::{path::Path, ffi::OsString};
    use image::{io::Reader as Imagereader, imageops::FilterType, GenericImageView};
    use std::fs::{File, self};
    use std::{thread, time::Duration};
    fn read_image(path:&Path,scale:u32)->String{
        //根据传进来的图片路径读取图片
        let img = Imagereader::open(path)
                                .expect("无法打开文件，请确认路径与文件名是否正确")
                                .decode().expect("无法解码,请确认图片格式是否支持"); 
        let img = img.resize(
            (img.width() /scale) as u32,
            (img.height() /scale) as u32,
            FilterType::Nearest,
        ).grayscale();   
        let pallete = [' ', '.', '\\', '*', '#', '$', '@'];    
        let mut current_line = 0;    
        let mut result = String::new();
    
        for (_, line, rgba) in img.pixels() {        
            if current_line != line 
            {
                result.push('\n');
                current_line = line;
            }        
            //加权平均值法将rgb转换为灰度
            let r = 0.2126 * (rgba.0[0] as f32);        
            let g = 0.7152 * (rgba.0[0] as f32);        
            let b = 0.0722 * (rgba.0[0] as f32);        
            let gray = r + g + b;        
            let caracter = ((gray / 255.0) * (pallete.len() - 1) as f32).round() as usize;
    
            result.push(pallete[caracter]);        // 填充一下，有些扁
            if caracter < (pallete.len() - 2) {
                result.push('.');
            } else {
                result.push(' ');
            }
        }
    
        result
    }
    pub fn build(){
        //图片文件夹路径
        let image_dir = Path::new("/home/chenzhangtao/图片/image");
        //图片名数组
        let mut image_name = Vec::<OsString>::new();
        //读取文件，并将文件名加入文件名数组中
        for image_path in image_dir.read_dir().expect("read dir failed"){
            if let Ok(path) = image_path{
                image_name.push(path.file_name());
            }
        }
        //将文件名排序
        image_name.sort();

        //将每个图片转换为字符串并保存到文本文件中
        let text_dir = Path::new("/home/chenzhangtao/文档/text");
        let total = image_name.len();
        for (index,name) in image_name.iter().enumerate(){
            print!("{index}/{total}");
            print!("\x1b[2J");
            print!("\x1b[H");
            //将图片内容处理成字符串后返回并写入文本文件中
            fs::write(
                text_dir.join(Path::new(&name).file_stem().unwrap()),
            read_image(&image_dir.join(name), 
               50)).expect("write to {name} failed");
        }
    }
    //将创建好的文本循环打印
    pub fn show(delay:u64){
        //倒入文本所在目录
        let text_dir = Path::new("/home/chenzhangtao/文档/text");
        //文本名存储数组
        let mut text_name = Vec::<OsString>::new();
        //获取目录下所有文本名
        for text_path in text_dir.read_dir().expect("读取目录信息失败"){
            if let Ok(path) = text_path{
                text_name.push(path.file_name());
            }
        }
        //将文本名排序
        text_name.sort();
        //按dalay每隔delay毫秒打印一个文本
        let mut pr = String::new();
        for i in text_name{
            pr.clear();
            thread::sleep(Duration::from_millis(delay));
            File::open(text_dir.join(i)).expect("读取文件失败").read_to_string(&mut pr).unwrap();
            print!("{}",pr);
            print!("\x1b[2J");
            print!("\x1b[H");
        }
    }
}