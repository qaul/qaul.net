
package net.qaul.qaul;

import android.net.wifi.WifiManager;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/**
 * This class wraps WifiManager to provide access to the new Ad-hoc methods thru
 * reflection, until the API is made official.
 */

public class WifiManagerNew {
    private final WifiManager wifiManager;
    private Class<?> wc;

    public WifiManagerNew(WifiManager wm) {
        wifiManager = wm;
        wc = wm.getClass();
    }

    public boolean isIbssSupported() {
        try {
            Method m = wc.getMethod("isIbssSupported", (Class[]) null);
            Object ret = m.invoke(wifiManager, (Object[]) null);
            return (Boolean) ret;
            /* if this method does not exist, IBSS mode is also not supported */
        } catch (NoSuchMethodException e) {
            return false;
        } catch (IllegalArgumentException e) {
            return false;
        } catch (IllegalAccessException e) {
            return false;
        } catch (InvocationTargetException e) {
            return false;
        }
    }
}
