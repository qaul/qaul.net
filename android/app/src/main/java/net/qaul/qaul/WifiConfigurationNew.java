
package net.qaul.qaul;

import android.net.wifi.WifiConfiguration;
import android.os.Build;
import android.util.Log;

import java.lang.reflect.*;
import java.net.InetAddress;
import java.util.ArrayList;

/**
 * This class extends WifiConfiguration to provide access to the new Ad-hoc
 * configuration options and static IP configuration thru reflection, until the
 * API is made official.
 */

@SuppressWarnings("unchecked")

public class WifiConfigurationNew extends WifiConfiguration 
{
	public static final String MSG_TAG = "WifiConfigurationNew";
	
    private Object ipConfiguration;
	private Class<?> wc;

    public WifiConfigurationNew() 
    {
        wc = this.getClass();
    }

    public void setIsIBSS(boolean val) 
    		throws NoSuchFieldException, IllegalArgumentException,
            IllegalAccessException 
    {
        Field fIsIBSS = wc.getField("isIBSS");
        fIsIBSS.set(this, val);
    }

    public void setFrequency(int freq) 
    		throws NoSuchFieldException, IllegalArgumentException,
            IllegalAccessException 
    {
        Field fFreq = wc.getField("frequency");
        fFreq.set(this, freq);
    }

    
    /**
     * method to set IP address in Android >= Lollipop
     */
    public void setStaticIpLollipop(
    		InetAddress ipAddress, int prefixLength, InetAddress gateway, InetAddress[] dns
    		) 
    		throws ClassNotFoundException, IllegalAccessException, IllegalArgumentException, 
    		InvocationTargetException, NoSuchMethodException, NoSuchFieldException, InstantiationException 
    {
        
    	// set IpAssignment to STATIC.
        Object ipConfiguration = this.getClass().getMethod("getIpConfiguration").invoke(this);
    	setEnumField(ipConfiguration, "STATIC", "ipAssignment");

        // Then set properties in StaticIpConfiguration.
        Object staticIpConfig = newInstance("android.net.StaticIpConfiguration");
        Object linkAddress = newInstance("android.net.LinkAddress", new Class<?>[] { InetAddress.class, int.class }, new Object[] { ipAddress, prefixLength });

        setField(staticIpConfig, "ipAddress", linkAddress);
        setField(staticIpConfig, "gateway", gateway);
        getField(staticIpConfig, "dnsServers", ArrayList.class).clear();
        for (int i = 0; i < dns.length; i++)
            getField(staticIpConfig, "dnsServers", ArrayList.class).add(dns[i]);

        // set IP configuration
        Class myClass = this.getClass();
        Class staticIpConfigClass = Class.forName("android.net.StaticIpConfiguration");
        Method setConfigMethod = myClass.getMethod("setStaticIpConfiguration", staticIpConfigClass);
        setConfigMethod.invoke(this, staticIpConfig);
        
        //manager.updateNetwork(config);
        //manager.saveConfiguration();
    }
    
    
    /**
     * Set IP address to STATIC in android < Lollipop
     */
    public void setIpAssignment(String assign) 
    		throws SecurityException, IllegalArgumentException,
            NoSuchFieldException, IllegalAccessException 
    {

        ipConfiguration = this;
    	setEnumField(ipConfiguration, assign, "ipAssignment");
    }
    
    public void setIpAddress(InetAddress addr, int prefixLength) 
    		throws SecurityException, IllegalArgumentException, NoSuchFieldException, 
    		IllegalAccessException, NoSuchMethodException, ClassNotFoundException, 
    		InstantiationException, InvocationTargetException 
    {
    	Object linkProperties = getField(ipConfiguration, "linkProperties");
        if (linkProperties == null)
        {
        	Log.i(MSG_TAG, "setIpAddress: linkProperty field not found");
        	return;
        }
        Class<?> laClass = Class.forName("android.net.LinkAddress");
        Constructor<?> laConstructor = laClass.getConstructor(new Class[] {
                InetAddress.class, int.class
        });
        Object linkAddress = laConstructor.newInstance(addr, prefixLength);

        ArrayList<Object> mLinkAddresses = (ArrayList<Object>) getDeclaredField(linkProperties, "mLinkAddresses");
        mLinkAddresses.clear();
        mLinkAddresses.add(linkAddress);    		
    }

    public void setGateway(InetAddress gateway) 
    		throws SecurityException, IllegalArgumentException,
            NoSuchFieldException, IllegalAccessException, ClassNotFoundException,
            NoSuchMethodException, InstantiationException, InvocationTargetException 
    {
		Object linkProperties = getField(ipConfiguration, "linkProperties");
		if (linkProperties == null)
        {
        	Log.i(MSG_TAG, "setGateway: linkProperty field not found");
        	return;
        }
        Class<?> routeInfoClass = Class.forName("android.net.RouteInfo");
        Constructor<?> routeInfoConstructor = routeInfoClass.getConstructor(new Class[] {
                InetAddress.class
        });
        Object routeInfo = routeInfoConstructor.newInstance(gateway);

        ArrayList<Object> mRoutes = (ArrayList<Object>) getDeclaredField(linkProperties, "mRoutes");
        mRoutes.clear();
        mRoutes.add(routeInfo);
    }

    public void setDNS(InetAddress dns)
            throws SecurityException, IllegalArgumentException, NoSuchFieldException,
            IllegalAccessException 
    {
        Object linkProperties = getField(ipConfiguration, "linkProperties");
        if (linkProperties == null)
            return;

        ArrayList<InetAddress> mDnses =
                (ArrayList<InetAddress>) getDeclaredField(linkProperties, "mDnses");
        mDnses.clear();
        mDnses.add(dns);
    }

    /* reflection access helpers */
    private static Object getField(Object obj, String name)
            throws SecurityException, NoSuchFieldException, IllegalArgumentException,
            IllegalAccessException 
    {
        Field f = obj.getClass().getField(name);
        Object out = f.get(obj);
        return out;
    }

    private static Object getDeclaredField(Object obj, String name)
            throws SecurityException, NoSuchFieldException,
            IllegalArgumentException, IllegalAccessException 
    {
        Field f = obj.getClass().getDeclaredField(name);
        f.setAccessible(true);
        Object out = f.get(obj);
        return out;
    }

    private static void setEnumField(Object obj, String value, String name)
            throws SecurityException, NoSuchFieldException, IllegalArgumentException,
            IllegalAccessException 
    {
        Field f = obj.getClass().getField(name);
        f.set(obj, Enum.valueOf((Class<Enum>) f.getType(), value));
    }
    
    /* lollipop reflection helpers */
    private static Object newInstance(String className) 
    		throws ClassNotFoundException, InstantiationException, IllegalAccessException, 
    		NoSuchMethodException, IllegalArgumentException, InvocationTargetException
    {
        return newInstance(className, new Class<?>[0], new Object[0]);
    }

    private static Object newInstance(String className, Class<?>[] parameterClasses, Object[] parameterValues) 
    		throws NoSuchMethodException, InstantiationException, IllegalAccessException, 
    		IllegalArgumentException, InvocationTargetException, ClassNotFoundException
    {
        Class<?> clz = Class.forName(className);
        Constructor<?> constructor = clz.getConstructor(parameterClasses);
        return constructor.newInstance(parameterValues);
    }

    @SuppressWarnings({ "unchecked", "rawtypes" })
    private static Object getEnumValue(String enumClassName, String enumValue) 
    		throws ClassNotFoundException
    {
        Class<Enum> enumClz = (Class<Enum>)Class.forName(enumClassName);
        return Enum.valueOf(enumClz, enumValue);
    }

    private static void setField(Object object, String fieldName, Object value) 
    		throws IllegalAccessException, IllegalArgumentException, NoSuchFieldException
    {
        Field field = object.getClass().getDeclaredField(fieldName);
        field.set(object, value);
    }

    private static <T> T getField(Object object, String fieldName, Class<T> type) 
    		throws IllegalAccessException, IllegalArgumentException, NoSuchFieldException
    {
        Field field = object.getClass().getDeclaredField(fieldName);
        return type.cast(field.get(object));
    }
}
