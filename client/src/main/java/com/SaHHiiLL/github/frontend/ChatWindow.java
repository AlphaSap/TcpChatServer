package com.SaHHiiLL.github.frontend;

import com.SaHHiiLL.github.ClientConnection;
import com.SaHHiiLL.github.ServerMessages;
import com.google.gson.Gson;

import javax.swing.*;
import javax.swing.text.*;
import java.awt.*;
import java.awt.event.ActionEvent;
import java.net.InetSocketAddress;

public class ChatWindow {

    private JFrame frame;
    private JPanel panel = new JPanel();

    private JTextField textField;
    private JButton sendButton;

    private ClientConnection connection = new ClientConnection(new InetSocketAddress("127.0.0.1", 6969));

    // add a scrollable text are to the panel
    private JTextArea textArea = new JTextArea();

    public ChatWindow() {

        this.frame = new JFrame("Chat");
        this.frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(500, 500);
        frame.setResizable(false);
        frame.add(panel);
        panel.setLayout(null);

        // stick the text field to the bottom and send button next to it
        textField = new JTextField();
        textField.setBounds(10, 420, 400, 30);
        panel.add(textField);
        sendButton = new JButton("Send");
        sendButton.setBounds(420, 420, 70, 30);
        panel.add(sendButton);

        // add the text area to the panel
        textArea.setBounds(10, 10, 480, 400);
        textArea.setEditable(false);

        //make text area scrollable
        connection.update(textArea);
        JScrollPane scrollPane = new JScrollPane(textArea);
        scrollPane.setBounds(10, 10, 480, 400);
        panel.add(scrollPane);
        Action action = new AbstractAction()
        {
            @Override
            public void actionPerformed(ActionEvent actionEvent) {
                String msg  = textField.getText();
                String addr = connection.getLocalHost();
                ServerMessages s_msg = new ServerMessages(addr, msg);
                connection.send(s_msg);
                textArea.append("You: " + textField.getText() + "\n");
                textField.setText("");
            }
        };

        sendButton.addActionListener(action);
        textField.addActionListener(action);
        frame.setVisible(true);
    }
}
