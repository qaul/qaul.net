
package net.qaul.qaul;

import android.net.wifi.WifiConfiguration;

import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.net.InetAddress;
import java.util.ArrayList;

/**
 * This class extends WifiConfiguration to provide access to the new Ad-hoc
 * configuration options and static IP configuration thru reflection, until the
 * API is made official.
 */

@SuppressWarnings("unchecked")

public class WifiConfigurationNew extends WifiConfiguration {
    private Class<?> wc;

    public WifiConfigurationNew() {
        wc = this.getClass();
    }

    public void setIsIBSS(boolean val) throws NoSuchFieldException, IllegalArgumentException,
            IllegalAccessException {
        Field fIsIBSS = wc.getField("isIBSS");
        fIsIBSS.set(this, val);
    }

    public void setFrequency(int freq) throws NoSuchFieldException, IllegalArgumentException,
            IllegalAccessException {
        Field fFreq = wc.getField("frequency");
        fFreq.set(this, freq);
    }

    public void setIpAssignment(String assign) throws SecurityException, IllegalArgumentException,
            NoSuchFieldException, IllegalAccessException {
        setEnumField(this, assign, "ipAssignment");
    }

    public void setIpAddress(InetAddress addr, int prefixLength) throws SecurityException,
            IllegalArgumentException, NoSuchFieldException, IllegalAccessException,
            NoSuchMethodException, ClassNotFoundException, InstantiationException,
            InvocationTargetException {
        Object linkProperties = getField(this, "linkProperties");
        if (linkProperties == null)
            return;
        Class<?> laClass = Class.forName("android.net.LinkAddress");
        Constructor<?> laConstructor = laClass.getConstructor(new Class[] {
                InetAddress.class, int.class
        });
        Object linkAddress = laConstructor.newInstance(addr, prefixLength);

        ArrayList<Object> mLinkAddresses = (ArrayList<Object>) getDeclaredField(linkProperties, "mLinkAddresses");
        mLinkAddresses.clear();
        mLinkAddresses.add(linkAddress);
    }

    public void setGateway(InetAddress gateway) throws SecurityException, IllegalArgumentException,
            NoSuchFieldException, IllegalAccessException, ClassNotFoundException,
            NoSuchMethodException, InstantiationException, InvocationTargetException {
        Object linkProperties = getField(this, "linkProperties");
        if (linkProperties == null)
            return;
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
            IllegalAccessException {
        Object linkProperties = getField(this, "linkProperties");
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
            IllegalAccessException {
        Field f = obj.getClass().getField(name);
        Object out = f.get(obj);
        return out;
    }

    private static Object getDeclaredField(Object obj, String name)
            throws SecurityException, NoSuchFieldException,
            IllegalArgumentException, IllegalAccessException {
        Field f = obj.getClass().getDeclaredField(name);
        f.setAccessible(true);
        Object out = f.get(obj);
        return out;
    }

    private static void setEnumField(Object obj, String value, String name)
            throws SecurityException, NoSuchFieldException, IllegalArgumentException,
            IllegalAccessException {
        Field f = obj.getClass().getField(name);
        f.set(obj, Enum.valueOf((Class<Enum>) f.getType(), value));
    }
}
