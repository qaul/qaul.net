/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

package net.qaul.qaul;

import java.io.File;

import android.app.Activity;
import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.util.Log;
import android.webkit.MimeTypeMap;
import android.webkit.WebView;

import com.lamerman.FileDialog;
import com.lamerman.SelectionMode;

public class QaulActivity extends Activity {	
	// Called each time when the activity (the view) is created.
    @Override
    public void onCreate(Bundle savedInstanceState) {
    	super.onCreate(savedInstanceState);
        setContentView(R.layout.main);
        
        // set reference to QaulApplication
        QaulApplication appState = ((QaulApplication)getApplicationContext());
    	appState.qaulSetActivity(this);
    	
        Log.i("QaulActivity", "before webview");
        
        WebView mainWebView = (WebView) findViewById(R.id.mainWebView);
        mainWebView.getSettings().setJavaScriptEnabled(true);
        mainWebView.loadUrl("http://localhost:8081/qaul.html");

        Log.i("QaulActivity", "after webview");
    }

    // The activity is about to be destroyed.
    @Override
    protected void onDestroy() {
    	QaulApplication appState = ((QaulApplication)getApplicationContext());
    	appState.qaulRemoveActivity();
    	
    	super.onDestroy();
    }

    // preventing back button from closing the application
    @Override
    public void onBackPressed() {
    	// TODO: integrate gui back behaviour
    	moveTaskToBack(true);
    }
    
    public void openFilePicker()
    {
    	Intent intent = new Intent(getBaseContext(), FileDialog.class);
        intent.putExtra(FileDialog.START_PATH, "/sdcard");
        
        //can user select directories or not
        intent.putExtra(FileDialog.CAN_SELECT_DIR, false);
        
        //alternatively you can set file filter
        //intent.putExtra(FileDialog.FORMAT_FILTER, new String[] { "png" });
        
    	startActivityForResult(intent, SelectionMode.MODE_OPEN);
    }

    public void openFile(File myFile)
    {
    	Log.i("QaulActivity", "open file");
    	Intent intent = new Intent();
        intent.setAction(android.content.Intent.ACTION_VIEW);
        
        MimeTypeMap mime = MimeTypeMap.getSingleton();
        String ext= myFile.getName().substring(myFile.getName().lastIndexOf(".")+1);
        String type = mime.getMimeTypeFromExtension(ext);
        
        Log.i("QaulActivity", "Mime Type: " +type);
      
        intent.setDataAndType(Uri.fromFile(myFile),type);        
        Log.i("QaulActivity", "openFile 1");
        startActivity(intent);
    	Log.i("QaulActivity", "openFile 2");
    }
    
    public synchronized void onActivityResult(final int requestCode,
            int resultCode, final Intent data) {

            if (resultCode == Activity.RESULT_OK) {
            	Log.i("QaulActivity", "returned path success");
            	String filePath = data.getStringExtra(FileDialog.RESULT_PATH);
            	Log.i("QaulActivity", "path: "+filePath);
            	// send path to qaullib
            	QaulApplication appState = ((QaulApplication)getApplicationContext());
            	appState.nativeQaul.filePicked(2, filePath);

            } else if (resultCode == Activity.RESULT_CANCELED) {
            	Log.i("QaulActivity", "returned no path");
            	QaulApplication appState = ((QaulApplication)getApplicationContext());
            	appState.nativeQaul.filePicked(0, "");
            	// log error    
            	//Logger.getLogger(AccelerationChartRun.class.getName()).log(
                //                Level.WARNING, "file not selected");
            }

    }}