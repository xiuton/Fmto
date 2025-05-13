mod gui;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use egui::ViewportBuilder;

use Fmto::{ConfigConverterFactory, ConfigFormat};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输入文件路径
    #[arg(short = 'i', long)]
    input: Option<PathBuf>,

    /// 输出文件路径（可选，可以指定多个）
    #[arg(short = 'o', long, num_args = 1..)]
    output: Vec<PathBuf>,

    /// 输出目录
    #[arg(short = 'd', long)]
    output_dir: Option<PathBuf>,

    /// 输入文件格式
    #[arg(short = 'f', long)]
    input_format: Option<String>,

    /// 输出文件格式（可选，可以指定多个，与输出文件一一对应）
    #[arg(short = 't', long, num_args = 1..)]
    output_format: Vec<String>,

    /// 启动图形界面
    #[arg(short = 'g', long)]
    gui: bool,
}

fn ensure_dir_exists(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 如果没有指定输入文件或指定了 GUI 模式，启动图形界面
    if args.input.is_none() || args.gui {
        let options = eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([800.0, 600.0]),
            ..Default::default()
        };
        
        // 使用 std::process::exit 来处理错误
        if let Err(e) = eframe::run_native(
            "Fmto - 配置文件格式转换工具",
            options,
            Box::new(|cc| Box::new(gui::FmtoApp::new(cc))),
        ) {
            eprintln!("GUI 错误: {}", e);
            std::process::exit(1);
        }
        return Ok(());
    }

    // 命令行模式
    let input = args.input.unwrap();

    // 从文件扩展名确定输入格式
    let input_format = args.input_format
        .map(|f| ConfigFormat::from_extension(&f))
        .flatten()
        .or_else(|| {
            input.extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| ConfigFormat::from_extension(ext))
        })
        .ok_or_else(|| anyhow::anyhow!("无法确定输入文件格式"))?;

    // 读取输入文件
    let content = std::fs::read_to_string(&input)?;

    // 获取输入转换器并解析
    let input_converter = ConfigConverterFactory::get_converter(input_format);
    let config = input_converter.parse(&content)?;

    // 确定输出文件列表
    let output_files = if !args.output.is_empty() {
        args.output
    } else if let Some(output_dir) = args.output_dir {
        // 如果指定了输出目录但没有指定输出文件，则使用输入文件名加上所有输出格式的扩展名
        let file_stem = input.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("无法获取输入文件名"))?;
        
        // 确保输出目录存在
        std::fs::create_dir_all(&output_dir)?;
        
        // 如果没有指定输出格式，则使用所有支持的格式
        let formats = if args.output_format.is_empty() {
            vec!["json", "yaml", "toml", "ini", "xml", "hocon", "env"]
                .into_iter()
                .map(String::from)
                .collect()
        } else {
            args.output_format.clone()
        };

        formats.into_iter()
            .map(|ext| output_dir.join(format!("{}.{}", file_stem, ext)))
            .collect()
    } else {
        // 如果没有指定输出文件或输出目录，则使用输入文件名加上输出格式的扩展名
        let file_stem = input.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("无法获取输入文件名"))?;
        
        // 如果没有指定输出格式，则使用输入格式
        let formats = if args.output_format.is_empty() {
            vec![input_format.to_extension().to_string()]
        } else {
            args.output_format.clone()
        };

        formats.into_iter()
            .map(|ext| PathBuf::from(format!("{}.{}", file_stem, ext)))
            .collect()
    };

    // 转换并写入所有输出文件
    for (i, output_path) in output_files.iter().enumerate() {
        // 确定输出格式
        let output_format = if i < args.output_format.len() {
            ConfigFormat::from_extension(&args.output_format[i])
        } else {
            output_path.extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| ConfigFormat::from_extension(ext))
        }.ok_or_else(|| anyhow::anyhow!("无法确定输出文件格式: {}", output_path.display()))?;

        // 获取输出转换器并格式化
        let output_converter = ConfigConverterFactory::get_converter(output_format);
        let output = output_converter.format(&config)?;

        // 确保输出目录存在
        ensure_dir_exists(output_path)?;

        // 写入输出文件
        std::fs::write(output_path, output)?;

        println!("已转换: {} -> {} ({})", 
            input.display(), 
            output_path.display(), 
            output_format.to_extension()
        );
    }

    println!("\n转换完成！共转换 {} 个文件", output_files.len());

    Ok(())
}
