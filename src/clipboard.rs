use arboard::Clipboard;

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut ctx = Clipboard::new()
        .map_err(|e| format!("Failed to access clipboard: {}", e))?;

    ctx.set_text(text)
        .map_err(|e| format!("Failed to set clipboard text: {}", e))?;

    Ok(())
}