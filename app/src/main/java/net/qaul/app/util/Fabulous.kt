package net.qaul.app.util

import android.view.View

/**
 * A utility function that rotates a FAB
 */
fun rotateFab(v: View, rotate: Boolean) {
    v.animate().setDuration(200)
            .rotation(if (rotate) 135f else 0f)
}

fun fanSubFabs(views: List<View>, spacing: Int, yOffset: Int) {
    views.forEachIndexed { idx, view ->
        view.animate()
                .setDuration(200)
                .translationYBy(((-yOffset * (idx + 1)) - spacing).toFloat())
    }
}

fun defanSubFabs(views: List<View>, y: Float) {
    for (v in views) {
        v.animate()
                .setDuration(200)
                .translationY(y)

    }
}