use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::{ConfigConverterFactory, ConfigFormat};
use eframe::egui::{FontDefinitions, FontFamily};
use std::process::Command;
use std::env;

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
            self.input_path = Some(path.clone());
            self.error = None;

            // 获取输入文件格式并取消对应格式的勾选
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if let Some(format) = ConfigFormat::from_extension(ext_str) {
                        let format_str = format.to_extension();
                        self.selected_formats.retain(|f| f != format_str);
                    }
                }
            }
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
                            let file_stem = "output"; // 使用固定的输出文件名

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

// 内置字体数据
const NOTO_SANS_SC_REGULAR: &[u8] = include_bytes!("../assets/fonts/NotoSansSC-Regular.ttf");

impl eframe::App for FmtoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 加载字体
        let mut fonts = FontDefinitions::default();
        
        // 使用内置字体数据
        fonts.font_data.insert(
            "noto_sans_sc".to_owned(),
            egui::FontData::from_owned(NOTO_SANS_SC_REGULAR.to_vec()),
        );

        // 将中文字体设置为默认字体
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "noto_sans_sc".to_owned());

        // 应用字体设置
        ctx.set_fonts(fonts);

        // 重置并设置主题样式
        let mut style = egui::Style::default();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.window_margin = egui::style::Margin::same(0.0);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(245, 245, 245);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(235, 235, 235);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(225, 225, 225);
        style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(30, 30, 30);
        style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(20, 20, 20);
        style.visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(10, 10, 10);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // 背景色
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                0.0,
                egui::Color32::from_rgb(252, 252, 252),
            );

            // 标题区域
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(egui::RichText::new("Fmto").size(36.0).color(egui::Color32::from_rgb(25, 25, 25)));
                ui.label(egui::RichText::new("配置文件格式转换工具").size(18.0).color(egui::Color32::from_rgb(80, 80, 80)));
                ui.add_space(20.0);
            });

            // 主要内容区域
            ui.vertical_centered(|ui| {
                let max_width = 600.0;
                let available_width = ui.available_width();
                let margin = (available_width - max_width) / 2.0;
                
                ui.horizontal(|ui| {
                    ui.add_space(margin);
                    ui.vertical(|ui| {
                        // 文件选择区域
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(255, 255, 255))
                            .inner_margin(egui::style::Margin::same(16.0))
                            .rounding(8.0)
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 220, 220)))
                            .show(ui, |ui| {
                                // 输入文件选择
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("输入文件").size(16.0).color(egui::Color32::from_rgb(30, 30, 30)));
                                    ui.add_space(12.0);
                                    let button = egui::Button::new(egui::RichText::new("选择文件").size(14.0).color(egui::Color32::from_rgb(255, 255, 255)))
                                        .fill(egui::Color32::from_rgb(41, 128, 185))
                                        .min_size(egui::vec2(80.0, 32.0));
                                    if ui.add(button).clicked() {
                                        self.select_input_file();
                                    }
                                    if let Some(path) = &self.input_path {
                                        ui.add_space(8.0);
                                        let path_button = egui::Button::new(egui::RichText::new(path.display().to_string()).size(14.0).color(egui::Color32::from_rgb(41, 128, 185)))
                                            .fill(egui::Color32::from_rgb(240, 240, 240));
                                        if ui.add(path_button).clicked() {
                                            if let Some(parent) = path.parent() {
                                                let _ = Command::new("explorer")
                                                    .arg(parent)
                                                    .output();
                                            }
                                        }
                                    }
                                });
                                ui.add_space(12.0);

                                // 输出目录选择
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("输出目录").size(16.0).color(egui::Color32::from_rgb(30, 30, 30)));
                                    ui.add_space(12.0);
                                    let button = egui::Button::new(egui::RichText::new("选择目录").size(14.0).color(egui::Color32::from_rgb(255, 255, 255)))
                                        .fill(egui::Color32::from_rgb(41, 128, 185))
                                        .min_size(egui::vec2(80.0, 32.0));
                                    if ui.add(button).clicked() {
                                        self.select_output_dir();
                                    }
                                    if let Some(path) = &self.output_dir {
                                        ui.add_space(8.0);
                                        let path_button = egui::Button::new(egui::RichText::new(path.display().to_string()).size(14.0).color(egui::Color32::from_rgb(41, 128, 185)))
                                            .fill(egui::Color32::from_rgb(240, 240, 240));
                                        if ui.add(path_button).clicked() {
                                            let _ = Command::new("explorer")
                                                .arg(path)
                                                .output();
                                        }
                                    }
                                });
                            });

                        ui.add_space(16.0);

                        // 输出格式选择区域
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(255, 255, 255))
                            .inner_margin(egui::style::Margin::same(16.0))
                            .rounding(8.0)
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 220, 220)))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("输出格式").size(16.0).color(egui::Color32::from_rgb(30, 30, 30)));
                                ui.add_space(12.0);
                                
                                let formats = ["json", "yaml", "toml", "ini", "xml", "hocon", "env"];
                                ui.horizontal_wrapped(|ui| {
                                    for format in formats {
                                        let mut selected = self.selected_formats.contains(&format.to_string());
                                        if ui.checkbox(&mut selected, egui::RichText::new(format).size(14.0).color(egui::Color32::from_rgb(60, 60, 60))).changed() {
                                            if selected {
                                                self.selected_formats.push(format.to_string());
                                            } else {
                                                self.selected_formats.retain(|f| f != format);
                                            }
                                        }
                                        ui.add_space(12.0);
                                    }
                                });
                            });

                        ui.add_space(16.0);

                        // 转换按钮
                        ui.vertical_centered(|ui| {
                            let button = egui::Button::new(egui::RichText::new("开始转换").size(16.0).color(egui::Color32::from_rgb(255, 255, 255)))
                                .fill(egui::Color32::from_rgb(41, 128, 185))
                                .min_size(egui::vec2(200.0, 40.0));
                            
                            if ui.add(button).clicked() {
                                self.convert_files();
                            }
                        });

                        ui.add_space(16.0);

                        // 状态和错误信息区域
                        if !self.status.is_empty() || self.error.is_some() {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgb(255, 255, 255))
                                .inner_margin(egui::style::Margin::same(16.0))
                                .rounding(8.0)
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 220, 220)))
                                .show(ui, |ui| {
                                    if !self.status.is_empty() {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("状态：").size(14.0).color(egui::Color32::from_rgb(30, 30, 30)));
                                            ui.label(egui::RichText::new(&self.status).size(14.0).color(egui::Color32::from_rgb(60, 60, 60)));
                                        });
                                    }

                                    if let Some(error) = &self.error {
                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("错误：").size(14.0).color(egui::Color32::from_rgb(231, 76, 60)));
                                            ui.label(egui::RichText::new(error).size(14.0).color(egui::Color32::from_rgb(231, 76, 60)));
                                        });
                                    }
                                });
                        }
                    });
                    ui.add_space(margin);
                });
            });
        });
    }
} 