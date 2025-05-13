use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::{ConfigConverterFactory, ConfigFormat};

pub struct FmtoApp {
    input_path: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    selected_formats: Vec<String>,
    status: String,
    error: Option<String>,
}

impl Default for FmtoApp {
    fn default() -> Self {
        Self {
            input_path: None,
            output_dir: None,
            selected_formats: vec!["json".to_string(), "yaml".to_string(), "toml".to_string()],
            status: "就绪".to_string(),
            error: None,
        }
    }
}

impl FmtoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            input_path: None,
            output_dir: None,
            selected_formats: vec!["json".to_string(), "yaml".to_string(), "toml".to_string()],
            status: "就绪".to_string(),
            error: None,
        }
    }

    fn select_input_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("配置文件", &["json", "yaml", "yml", "toml", "ini", "xml", "conf", "env"])
            .pick_file() 
        {
            self.input_path = Some(path);
            self.error = None;
        }
    }

    fn select_output_dir(&mut self) {
        if let Some(path) = FileDialog::new()
            .pick_folder() 
        {
            self.output_dir = Some(path);
            self.error = None;
        }
    }

    fn convert_files(&mut self) {
        if let (Some(input_path), Some(output_dir)) = (&self.input_path, &self.output_dir) {
            // 获取输入格式
            let input_format = input_path.extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| ConfigFormat::from_extension(ext))
                .unwrap_or(ConfigFormat::Json);

            // 读取输入文件
            match std::fs::read_to_string(input_path) {
                Ok(content) => {
                    // 解析输入文件
                    let input_converter = ConfigConverterFactory::get_converter(input_format);
                    match input_converter.parse(&content) {
                        Ok(config) => {
                            let mut success_count = 0;
                            let file_stem = input_path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("config");

                            // 转换到每个选定的格式
                            for format in &self.selected_formats {
                                if let Some(output_format) = ConfigFormat::from_extension(format) {
                                    let output_path = output_dir.join(format!("{}.{}", file_stem, format));
                                    let output_converter = ConfigConverterFactory::get_converter(output_format);
                                    
                                    match output_converter.format(&config) {
                                        Ok(output) => {
                                            if let Err(e) = std::fs::write(&output_path, output) {
                                                self.error = Some(format!("写入文件失败: {}", e));
                                                continue;
                                            }
                                            success_count += 1;
                                        }
                                        Err(e) => {
                                            self.error = Some(format!("转换失败: {}", e));
                                            continue;
                                        }
                                    }
                                }
                            }

                            if success_count > 0 {
                                self.status = format!("成功转换 {} 个文件", success_count);
                            }
                        }
                        Err(e) => self.error = Some(format!("解析输入文件失败: {}", e)),
                    }
                }
                Err(e) => self.error = Some(format!("读取输入文件失败: {}", e)),
            }
        } else {
            self.error = Some("请选择输入文件和输出目录".to_string());
        }
    }
}

impl eframe::App for FmtoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fmto - 配置文件格式转换工具");
            ui.add_space(10.0);

            // 输入文件选择
            ui.horizontal(|ui| {
                ui.label("输入文件：");
                if ui.button("选择文件").clicked() {
                    self.select_input_file();
                }
                if let Some(path) = &self.input_path {
                    ui.label(path.display().to_string());
                }
            });

            // 输出目录选择
            ui.horizontal(|ui| {
                ui.label("输出目录：");
                if ui.button("选择目录").clicked() {
                    self.select_output_dir();
                }
                if let Some(path) = &self.output_dir {
                    ui.label(path.display().to_string());
                }
            });

            ui.add_space(10.0);

            // 输出格式选择
            ui.label("输出格式：");
            let formats = ["json", "yaml", "toml", "ini", "xml", "hocon", "env"];
            ui.horizontal_wrapped(|ui| {
                for format in formats {
                    let mut selected = self.selected_formats.contains(&format.to_string());
                    if ui.checkbox(&mut selected, format).changed() {
                        if selected {
                            self.selected_formats.push(format.to_string());
                        } else {
                            self.selected_formats.retain(|f| f != format);
                        }
                    }
                }
            });

            ui.add_space(10.0);

            // 转换按钮
            if ui.button("开始转换").clicked() {
                self.convert_files();
            }

            ui.add_space(10.0);

            // 状态显示
            ui.label(&self.status);

            // 错误显示
            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::RED, error);
            }
        });
    }
} 