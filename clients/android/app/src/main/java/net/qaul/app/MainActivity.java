package net.qaul.app;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.util.Log;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    static {
        // The "android-support" crate creates a dynamic library called "libqauldroid"
        // which we can include here simply via "qauldroid" because it's being put
        // into the library search path via ~ m a g i c ~
        System.loadLibrary("qauldroid");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        // Example of a call to a native method
        TextView tv = findViewById(R.id.sample_text);
        tv.setText(hello("qaul.net"));
    }

    public native String hello(String to);
}
