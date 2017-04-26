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

import java.io.BufferedReader;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.zip.GZIPInputStream;

public class Configuration {

	public static final String DEVICE_NEXUSONE   = "nexusone";
	public static final String DEVICE_GALAXY1X   = "galaxy1x";
	public static final String DEVICE_GALAXY2X   = "galaxy2x";
	public static final String DEVICE_LEGEND     = "legend";
	public static final String DEVICE_DREAM      = "dream";
	public static final String DEVICE_GENERIC    = "generic";
	
	public static final String DRIVER_TIWLAN0     = "tiwlan0";
	public static final String DRIVER_WEXT        = "wext";
	
//	/**
//	 * delete this method
//	 * 
//	 * Returns the device-type as string.
//	 * A very ugly hack - checking for wifi-kernel-modules.
//	 */
//	public static String getDeviceType() 
//	{
//		if ((new File("/system/lib/modules/bcm4329.ko")).exists() == true) {
//			return DEVICE_NEXUSONE;
//		}
//		if ((new File("/system/libmodules/bcm4325.ko")).exists() == true) 
//		{
//			int sdkVersion = Integer.parseInt(Build.VERSION.SDK);
//        	if (sdkVersion >= Build.VERSION_CODES.DONUT) {
//        		return DEVICE_GALAXY2X;
//        	}
//			return DEVICE_GALAXY1X;
//		}
//		else if ((new File("/system/lib/modules/tiwlan_drv.ko")).exists() == true 
//				&& (new File("/system/etc/wifi/Fw1273_CHIP.bin")).exists() == true) 
//		{
//			return DEVICE_LEGEND;
//		}
//		else if ((new File("/system/lib/modules/wlan.ko")).exists() == true) 
//		{
//			return DEVICE_DREAM;
//		}
//		return DEVICE_GENERIC;
//	}
	
	/**
	 * check which wifi chip this device uses
	 * 
	 * @return wifi module
	 */
	public static String getWifiModule() 
	{
		// Samsung Galaxy S2, Galaxy Tab
		if ((new File("/lib/modules/dhd.ko")).exists() == true)
			return "dhd";

		else if ((new File("/system/lib/modules/bcm4329.ko")).exists() == true)
			return "bcm4329";

		else if ((new File("/system/lib/modules/bcm4325.ko")).exists() == true) 
			return "bcm4325";
		
		else if ((new File("/system/lib/modules/tiwlan_drv.ko")).exists() == true)
			return "tiwlan_drv";
		
		else if ((new File("/system/lib/modules/wlan.ko")).exists() == true) 
			return "wlan";
					
		else if ((new File("/system/lib/modules/sdio.ko")).exists() == true) 
			return "sdio";
					
		return "unknown";
	}
	
	/**
	 * check which firmware this device uses
	 * 
	 * @return firmware file
	 */
	public static String getFirmwarePath() 
	{
		// HTC sensation
		if ((new File("/etc/firmware/fw_bcm4329.bin")).exists() == true) 
			return "/etc/firmware/fw_bcm4329.bin";
		
		// HTC Legend
		else if ((new File("/system/etc/wifi/Fw1273_CHIP.bin")).exists() == true) 
			return "/system/etc/wifi/Fw1273_CHIP.bin";
		
		// HTC dream/magic/tattoo/eris/hero
		else if ((new File("/system/etc/wifi/Fw1251r1c.bin")).exists() == true) 
			return "/system/etc/wifi/Fw1251r1c.bin";
		
		// Samsung Galaxy S2
		else if ((new File("/system/etc/wifi/bcm4330_sta.bin")).exists() == true) 
			return "/system/etc/wifi/bcm4330_sta.bin";
		
		return "unknown";
	}
	
	/**
	 * determine the nvram path
	 * 
	 * @return nvram_path
	 */
	public static String getNvramPath() 
	{
		// HTC Sensation
		if ((new File("/system/etc/calibration")).exists() == true) 
			return "/system/etc/calibration";

		else if ((new File("/system/etc/nvram.txt")).exists() == true) 
			return "/system/etc/nvram.txt";
		
		// Samsung Galaxy S2
		else if ((new File("/system/etc/wifi/nvram_net.txt")).exists() == true) 
			return "/system/etc/wifi/nvram_net.txt";
		
		return "unknown";
	}
	
	/**
	 * Returns the wpa_supplicant-driver which should be used
	 * on wpa_supplicant-start 
	 */
	public static String getWifiInterfaceDriver() 
	{
		if ((new File("/system/lib/modules/wlan.ko")).exists() == true) 
		{
			return "tiwlan0";
		}
		return "wext";
	}

//	/**
//	 * Returns the wpa_supplicant-driver which should be used
//	 * on wpa_supplicant-start 
//	 */
//	public static String getEncryptionAutoMethod(String deviceType) {
//		if (deviceType.equals(DEVICE_LEGEND) || deviceType.equals(DEVICE_NEXUSONE)) {
//			return "iwconfig";
//		}
//		return "wpa_supplicant";
//	}
	
	/**
	 * Returns a boolean if fix_persist.sh is required
	 * @param feature
	 * @return
	 */
	public static boolean enableFixPersist() 
	{
		if ((new File("/system/lib/modules/tiwlan_drv.ko")).exists() == true 
				&& (new File("/system/etc/wifi/fw_wlan1271.bin")).exists() == true
				&& getWifiInterfaceDriver().equals(DRIVER_WEXT) == true)
		{
			return true;
		}
		else if ((new File("/system/lib/modules/tiwlan_drv.ko")).exists() == true 
				&& (new File("/system/etc/wifi/Fw1273_CHIP.bin")).exists() == true) 
		{
			return true;
		}
		
		return false;
	}
	
	/**
	 * Returns a boolean if fix_persist.sh is required
	 * @param feature
	 * @return
	 */
	public static boolean enableFixRoute() 
	{
		if ((new File("/system/etc/iproute2/rt_tables")).exists() == true 
				&& NativeTask.getProp("ro.product.manufacturer").equalsIgnoreCase("HTC")) 
		{
			return true;
		}
		return false;
	}	
	
    public static boolean hasKernelFeature(String feature) 
    {
    	try {
			File cfg = new File("/proc/config.gz");
			if (cfg.exists() == false) 
			{
				return true;
			}
			FileInputStream fis = new FileInputStream(cfg);
			GZIPInputStream gzin = new GZIPInputStream(fis);
			BufferedReader in = null;
			String line = "";
			in = new BufferedReader(new InputStreamReader(gzin));
			while ((line = in.readLine()) != null) {
				   if (line.startsWith(feature)) {
					    gzin.close();
						return true;
					}
			}
			gzin.close();
    	} catch (IOException e) {
    		e.printStackTrace();
    	}
    	return false;
    }
}
