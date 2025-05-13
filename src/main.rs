use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use Fmto::{ConfigConverterFactory, ConfigFormat};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输入文件路径
    #[arg(short = 'i', long)]
    input: PathBuf,

    /// 输出文件路径（可选，如果指定了输出目录则忽略）
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// 输出目录
    #[arg(short = 'd', long)]
    output_dir: Option<PathBuf>,

    /// 输入文件格式
    #[arg(short = 'f', long)]
    input_format: Option<String>,

    /// 输出文件格式
    #[arg(short = 't', long)]
    output_format: Option<String>,
}

fn ensure_dir_exists(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 从文件扩展名确定格式
    let input_format = args.input_format
        .map(|f| ConfigFormat::from_extension(&f))
        .flatten()
        .or_else(|| {
            args.input.extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| ConfigFormat::from_extension(ext))
        })
        .ok_or_else(|| anyhow::anyhow!("无法确定输入文件格式"))?;

    let output_format = args.output_format
        .map(|f| ConfigFormat::from_extension(&f))
        .flatten()
        .or_else(|| {
            args.output.as_ref()
                .and_then(|p| p.extension())
                .and_then(|ext| ext.to_str())
                .and_then(|ext| ConfigFormat::from_extension(ext))
        })
        .ok_or_else(|| anyhow::anyhow!("无法确定输出文件格式"))?;

    // 读取输入文件
    let content = std::fs::read_to_string(&args.input)?;

    // 获取转换器并执行转换
    let input_converter = ConfigConverterFactory::get_converter(input_format);
    let output_converter = ConfigConverterFactory::get_converter(output_format);

    let config = input_converter.parse(&content)?;
    let output = output_converter.format(&config)?;

    // 确定输出路径
    let output_path = if let Some(output_dir) = args.output_dir {
        // 如果指定了输出目录，则使用输入文件名加上输出格式的扩展名
        let file_stem = args.input.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("无法获取输入文件名"))?;
        
        // 确保输出目录存在
        std::fs::create_dir_all(&output_dir)?;
        
        output_dir.join(format!("{}.{}", file_stem, output_format.to_extension()))
    } else if let Some(output) = args.output {
        // 确保输出文件的父目录存在
        ensure_dir_exists(&output)?;
        output
    } else {
        // 如果没有指定输出路径或输出目录，则使用输入文件名加上输出格式的扩展名
        let file_stem = args.input.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("无法获取输入文件名"))?;
        
        PathBuf::from(format!("{}.{}", file_stem, output_format.to_extension()))
    };

    // 写入输出文件
    std::fs::write(&output_path, output)?;

    println!("转换完成！");
    println!("输入: {} ({})", args.input.display(), input_format.to_extension());
    println!("输出: {} ({})", output_path.display(), output_format.to_extension());

    Ok(())
}
