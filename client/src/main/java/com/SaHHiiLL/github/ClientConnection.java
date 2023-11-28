package com.SaHHiiLL.github;

import com.google.gson.Gson;

import javax.swing.*;
import java.awt.*;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.net.Socket;
import java.net.SocketAddress;
import java.util.Arrays;
import java.util.concurrent.CompletableFuture;

public class ClientConnection {

    private Socket connection;
    private PrintWriter out;
    private BufferedReader in;
    private Gson gson = new Gson();


    public ClientConnection(SocketAddress address) {
        try {
            connection = new Socket();
            connection.connect(address);
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
        return connection.getInetAddress().getHostAddress();
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
}
