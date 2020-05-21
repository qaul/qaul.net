package net.qaul.app;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    protected long libqaulState;

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

        // Start the libqaul machinery under the hood
        this.libqaulState = this.startServer(5000, "");
    }

    public native String hello(String to);

    public native long startServer(int port, String path);
}
