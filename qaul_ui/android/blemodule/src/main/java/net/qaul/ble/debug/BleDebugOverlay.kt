package net.qaul.ble.test.ble.debug

import android.annotation.SuppressLint
import android.content.Context
import android.content.Intent
import android.graphics.Color
import android.graphics.Typeface
import android.net.Uri
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.provider.Settings
import android.util.Log
import android.util.TypedValue
import android.view.Gravity
import android.view.MotionEvent
import android.view.View
import android.view.WindowManager
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import net.qaul.ble.test.ble.connection.ConnectionPool

/**
 * Floating, draggable, collapsible debug overlay that shows live BLE stats (neighbours, radio state,
 * scan-result counts) on the device itself — so multi-phone testing doesn't require adb logcat.
 *
 * Drawn as a system overlay (TYPE_APPLICATION_OVERLAY) so it stays on top even if the qaul app is
 * backgrounded. Requires the one-time "Draw over other apps" permission; [show] launches the grant
 * screen if it isn't held yet and returns — call it again (or restart BLE) once granted.
 *
 * Debug-only. Gated by BleConstants.DEBUG_OVERLAY and wired from BleWrapperClass start/stop.
 */
@SuppressLint("StaticFieldLeak")   // we hold only the application context, never an Activity
object BleDebugOverlay {

    private const val TAG = "BleDebugOverlay"
    private const val REFRESH_MS = 1500L

    private var appContext: Context? = null
    private var windowManager: WindowManager? = null
    private var root: LinearLayout? = null
    private var pill: TextView? = null
    private var panel: LinearLayout? = null
    private var body: TextView? = null
    private var layoutParams: WindowManager.LayoutParams? = null

    private var expanded = false
    private val handler = Handler(Looper.getMainLooper())
    private val refresh = object : Runnable {
        override fun run() {
            update()
            handler.postDelayed(this, REFRESH_MS)
        }
    }

    /** Show the overlay. If the overlay permission isn't granted, opens the grant screen and returns. */
    fun show(context: Context) {
        val app = context.applicationContext
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && !Settings.canDrawOverlays(app)) {
            Log.w(TAG, "Overlay permission not granted — opening settings. Re-start BLE once granted.")
            try {
                context.startActivity(
                    Intent(
                        Settings.ACTION_MANAGE_OVERLAY_PERMISSION,
                        Uri.parse("package:${app.packageName}")
                    ).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                )
            } catch (e: Exception) {
                Log.e(TAG, "Could not open overlay permission settings", e)
            }
            return
        }
        handler.post {
            if (root != null) return@post           // already showing
            build(app)
            handler.removeCallbacks(refresh)
            handler.post(refresh)
        }
    }

    fun hide() {
        handler.post {
            handler.removeCallbacks(refresh)
            root?.let { r -> try { windowManager?.removeView(r) } catch (_: Exception) {} }
            root = null; pill = null; panel = null; body = null; layoutParams = null
        }
    }

    private fun Context.dp(value: Int): Int = TypedValue.applyDimension(
        TypedValue.COMPLEX_UNIT_DIP, value.toFloat(), resources.displayMetrics
    ).toInt()

    @SuppressLint("ClickableViewAccessibility")
    private fun build(app: Context) {
        appContext = app
        windowManager = app.getSystemService(Context.WINDOW_SERVICE) as WindowManager

        val container = LinearLayout(app).apply { orientation = LinearLayout.VERTICAL }

        // Collapsed pill
        pill = TextView(app).apply {
            text = "BLE"
            setTextColor(Color.WHITE)
            setBackgroundColor(0xDD1565C0.toInt())
            setPadding(app.dp(12), app.dp(6), app.dp(12), app.dp(6))
            textSize = 12f
            typeface = Typeface.DEFAULT_BOLD
        }

        // Expanded panel: header (tap to collapse) + scrollable monospace body
        val header = TextView(app).apply {
            text = "BLE debug  ▾"
            setTextColor(Color.WHITE)
            setBackgroundColor(0xFF1565C0.toInt())
            setPadding(app.dp(12), app.dp(6), app.dp(12), app.dp(6))
            textSize = 12f
            typeface = Typeface.DEFAULT_BOLD
        }
        body = TextView(app).apply {
            setTextColor(Color.WHITE)
            setBackgroundColor(0xCC000000.toInt())
            setPadding(app.dp(12), app.dp(8), app.dp(12), app.dp(10))
            textSize = 11f
            typeface = Typeface.MONOSPACE
        }
        val scroll = ScrollView(app).apply {
            addView(body)
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, app.dp(220)
            )
        }
        panel = LinearLayout(app).apply {
            orientation = LinearLayout.VERTICAL
            addView(header)
            addView(scroll)
        }

        container.addView(pill)
        container.addView(panel)
        root = container

        val type = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
            WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY
        else @Suppress("DEPRECATION") WindowManager.LayoutParams.TYPE_PHONE

        val lp = WindowManager.LayoutParams(
            app.dp(300),
            WindowManager.LayoutParams.WRAP_CONTENT,
            type,
            // Not focusable so we never steal keyboard/input from the qaul app; touch on the view itself
            // still works for drag + collapse/expand.
            WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE,
            android.graphics.PixelFormat.TRANSLUCENT
        ).apply {
            gravity = Gravity.TOP or Gravity.START
            x = app.dp(8)
            y = app.dp(80)
        }
        layoutParams = lp

        val dragToggle = makeDragToggleListener()
        pill?.setOnTouchListener(dragToggle)
        header?.setOnTouchListener(dragToggle)

        try {
            windowManager?.addView(container, lp)
        } catch (e: Exception) {
            Log.e(TAG, "Failed to add overlay view", e)
            root = null
            return
        }
        render()
        update()
    }

    /** One listener handling both dragging the window and tap-to-toggle (drag wins past the touch slop). */
    private fun makeDragToggleListener(): View.OnTouchListener {
        var downX = 0f; var downY = 0f
        var startX = 0; var startY = 0
        var dragging = false
        val slop = (appContext?.dp(8) ?: 24)
        return View.OnTouchListener { _, event ->
            val lp = layoutParams ?: return@OnTouchListener false
            when (event.action) {
                MotionEvent.ACTION_DOWN -> {
                    downX = event.rawX; downY = event.rawY
                    startX = lp.x; startY = lp.y
                    dragging = false
                    true
                }
                MotionEvent.ACTION_MOVE -> {
                    val dx = (event.rawX - downX).toInt()
                    val dy = (event.rawY - downY).toInt()
                    if (!dragging && (kotlin.math.abs(dx) > slop || kotlin.math.abs(dy) > slop)) dragging = true
                    if (dragging) {
                        lp.x = startX + dx
                        lp.y = startY + dy
                        try { windowManager?.updateViewLayout(root, lp) } catch (_: Exception) {}
                    }
                    true
                }
                MotionEvent.ACTION_UP -> {
                    if (!dragging) { expanded = !expanded; render() }   // a tap, not a drag
                    true
                }
                else -> false
            }
        }
    }

    private fun render() {
        pill?.visibility = if (expanded) View.GONE else View.VISIBLE
        panel?.visibility = if (expanded) View.VISIBLE else View.GONE
    }

    private fun update() {
        if (root == null) return
        try {
            if (expanded) body?.text = ConnectionPool.debugStatusText()
            else pill?.text = ConnectionPool.debugSummary()
        } catch (e: Exception) {
            Log.e(TAG, "overlay update failed", e)
        }
    }
}
