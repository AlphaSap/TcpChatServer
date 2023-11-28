package com.SaHHiiLL.github;

import com.SaHHiiLL.github.frontend.ChatWindow;
import com.formdev.flatlaf.themes.FlatMacDarkLaf;

import javax.swing.*;

public class Main {
    public static void main(String[] args) throws UnsupportedLookAndFeelException {
        setTheme();
        SwingUtilities.invokeLater(ChatWindow::new);
    }

    static void setTheme() throws UnsupportedLookAndFeelException {
        FlatMacDarkLaf.setup();
        UIManager.setLookAndFeel(new FlatMacDarkLaf());
    }
}