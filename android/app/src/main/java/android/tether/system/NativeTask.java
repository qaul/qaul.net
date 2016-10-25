/*
 *  This program is free software; you can redistribute it and/or modify it under 
 *  the terms of the GNU General Public License as published by the Free Software 
 *  Foundation; either version 3 of the License, or (at your option) any later 
 *  version.
 *  You should have received a copy of the GNU General Public License along with 
 *  this program; if not, see <http://www.gnu.org/licenses/>. 
 *  Use this application at your own risk.
 *
 *  Copyright (c) 2009 by Harald Mueller and Sofia Lemons.
 */

package android.tether.system;

import android.util.Log;

public class NativeTask {
    
	public static final String MSG_TAG = "TETHER -> NativeTask";

	static {
        try {
            Log.i(MSG_TAG, "Trying to load libnativetask.so");
            System.loadLibrary("nativetask");
        }
        catch (UnsatisfiedLinkError ule) {
            Log.e(MSG_TAG, "Could not load libnativetask.so");
        }
    }
    public static native String getProp(String name);
    public static native int runCommand(String command);
}
