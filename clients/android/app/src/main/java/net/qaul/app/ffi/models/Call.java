package net.qaul.app.ffi.models;

import java.util.ArrayList;


/**
 *  A voice call between two or more people
 */
public class Call {
    public Id id;
    public ArrayList<Id> participants;
    public int startTime;

    public Call(Id id, ArrayList<Id> participants, int startTime) {
            this.id = id;
            this.participants = participants;
            this.startTime = startTime;
    }
}