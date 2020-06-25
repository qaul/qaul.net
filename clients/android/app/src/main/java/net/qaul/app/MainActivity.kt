package net.qaul.app

import android.os.Bundle
import com.google.android.material.bottomnavigation.BottomNavigationView
import androidx.appcompat.app.AppCompatActivity
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import net.qaul.app.util.AppState

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val navView: BottomNavigationView = findViewById(R.id.nav_view)

        val navCtrl = findNavController(R.id.nav_host_fragment)
        val appBarCfg = AppBarConfiguration(setOf(
                R.id.navigation_chat,
                R.id.navigation_files,
                R.id.navigation_users,
                R.id.navigation_settings))
        setupActionBarWithNavController(navCtrl, appBarCfg)
        navView.setupWithNavController(navCtrl)
    }
}
