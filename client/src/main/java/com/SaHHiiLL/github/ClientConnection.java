package com.SaHHiiLL.github;

import com.SaHHiiLL.github.frontend.ChatWindow;
import com.google.gson.Gson;

import javax.swing.*;
import javax.swing.text.Style;
import javax.swing.text.StyledDocument;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.net.InetSocketAddress;
import java.net.Socket;
import java.net.SocketAddress;
import java.util.concurrent.CompletableFuture;

public class ClientConnection {

    private Socket connection;
    private PrintWriter out;
    private BufferedReader in;
    private Gson gson = new Gson();

    private InetSocketAddress address;


    public ClientConnection(InetSocketAddress address) {
        this.address = address;
        try {
            connection = new Socket();
            connection.connect(this.address);
            out = new PrintWriter(connection.getOutputStream(), true);
            in = new BufferedReader(new InputStreamReader(connection.getInputStream()));
        } catch (Exception e) {
            // Log this thing, but not relly coz its the client
            e.printStackTrace();
        }
    }

    public void send(ServerMessages message) {
        String json = gson.toJson(message, ServerMessages.class);
        out.println(json);
    }

    public String getLocalHost() {
        return this.connection.getLocalSocketAddress().toString();
    }

    public BufferedReader getIn() {
        return in;
    }

    public void update(JTextArea textArea) {
        CompletableFuture.runAsync(() -> {
            try {
                String outputFromServer = "";
                while((outputFromServer=in.readLine())!= null){
                    ServerMessages serverMessages = gson.fromJson(outputFromServer, ServerMessages.class);
                    textArea.append(serverMessages.name + ": "+ serverMessages.message + "\n");
                }
            }
            catch (IOException e) {
                e.printStackTrace();
                throw new RuntimeException(e);
            }

        });
    }

    public void update(Style readStyle, StyledDocument document) {
        CompletableFuture.runAsync(() -> {
            try {
                String outputFromServer = "";
                while((outputFromServer=in.readLine())!= null){
                    System.out.println(outputFromServer);
                    ServerMessages serverMessages = gson.fromJson(outputFromServer, ServerMessages.class);
                    ChatWindow.appendToPane(serverMessages.name, readStyle, document);
                    ChatWindow.appendToPane(": " + serverMessages.message + "\n", null, document);
                }
            }
            catch (IOException e) {
                e.printStackTrace();
                throw new RuntimeException(e);
            }

        });
    }
}
