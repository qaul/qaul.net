/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

package net.qaul.qaul;

import android.util.Log;

public class NativeQaul 
{
	public static final String LOG_TAG = "NativeQaul";
	
	static 
	{
		try 
		{
			System.loadLibrary("qaul");
		}
		catch (UnsatisfiedLinkError ule)
		{
			Log.e(LOG_TAG, "Could not load libnativetask.so");
		}
	}

	public native void libInit(String resourcePath);
	public native void libExit();
	
	public native int webserverStart();
	public native void configStart();
	public native int captiveStart();
	
	public native int ipcConnect();
	public native int ipcClose();
	public native void ipcSendCom(int commandId);

	public native String getConfString(String key);
	public native int    getConfInt(String key);
	public native void   setConfString(String key, String value);
	public native void   setConfInt(String key, int value);	
	
	public native int   getNetProtocol();
	public native String getIP();
	public native int   getNetMask();
	public native String getNetGateway();
	public native String getWifiIbss();
	public native int   getWifiBssIdSet();
	public native String getWifiBssId();
	public native int   getWifiChannel();
	
	public native int existsUsername();
	public native void configurationFinished();

	public native void filePicked(int check, String path);
	public native String getAppEventOpenPath();
	public native String getAppEventOpenURL();
	
	public native int timedCheckAppEvent();
	public native void timedSocketReceive();
	public native void timedDownload();
}
