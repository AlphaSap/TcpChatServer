package com.SaHHiiLL.github;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

public class ServerMessages {
    @SerializedName("name")
            @Expose
    String name;
    @SerializedName("message")
    @Expose
    String message;

    public ServerMessages(String name, String message) {
        this.name = name;
        this.message = message;
    }

}
