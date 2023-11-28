package com.SaHHiiLL.github.frontend;

import com.SaHHiiLL.github.ClientConnection;
import com.SaHHiiLL.github.ServerMessages;

import javax.swing.*;
import javax.swing.text.*;
import java.awt.*;
import java.awt.event.ActionEvent;
import java.net.InetSocketAddress;

public class ChatWindow extends JFrame {
    private JTextPane textPane = new JTextPane();
    private StyledDocument document = textPane.getStyledDocument();
    private JPanel panel = new JPanel();
    private ClientConnection connection = new ClientConnection(new InetSocketAddress("127.0.0.1", 6969));

    private JTextField textField = new JTextField();
    private JButton sendButton = new JButton("Send message");

    private Style readStyle;
    private Style writeStyle;
    public ChatWindow() {
        setTitle("Socket GUI");
        setSize(500, 500);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setResizable(false);
        setLayout(null);

        // stick the text field to the bottom and send button next to it
        textField.setBounds(10, 420, 400, 30);
        add(textField);
        sendButton.setBounds(420, 420, 70, 30);
        add(sendButton);

        textPane.setBounds(10, 10, 480, 400);
        textPane.setEditable(false);

        JScrollPane scrollPane = new JScrollPane(textPane);
        scrollPane.setBounds(10, 10, 480, 400);
        add(scrollPane);

        sendButton.addActionListener(action);
        textField.addActionListener(action);

        // do the magic here
        Style defaultStyle = StyleContext.getDefaultStyleContext().getStyle(StyleContext.DEFAULT_STYLE);
        readStyle = document.addStyle("ReadStyle", defaultStyle);
        StyleConstants.setForeground(readStyle, Color.GREEN);

        writeStyle = document.addStyle("WriteStyle", defaultStyle);
        StyleConstants.setForeground(writeStyle, Color.RED);
        connection.update(readStyle, document);

        setVisible(true);
    }

    Action action = new AbstractAction()
    {
        @Override
        public void actionPerformed(ActionEvent actionEvent) {
            String msg  = textField.getText();
            String addr = connection.getLocalHost();
            ServerMessages s_msg = new ServerMessages(addr, msg);
            connection.send(s_msg);
            appendToPane("You", writeStyle, document);
            appendToPane(": " + msg + "\n", null, document);
            textField.setText("");
        }
    };
    public static void appendToPane(String text, Style style, StyledDocument document) {
        try {
            document.insertString(document.getLength(), text, style);
        } catch (BadLocationException e) {
            e.printStackTrace();
        }
    }


}
