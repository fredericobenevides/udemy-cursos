package com.bawp.activitylifecycle;

import androidx.annotation.Nullable;
import androidx.appcompat.app.AppCompatActivity;

import android.content.Intent;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.TextView;

public class ShowGuess extends AppCompatActivity {

    private TextView showGuessTextView;


    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_show_guess);

        Bundle extra = getIntent().getExtras();

        showGuessTextView = findViewById(R.id.received_textview);

        if (extra != null) {
            showGuessTextView.setText(extra.getString("guess"));
        }

        showGuessTextView.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                Intent intent = getIntent();
                intent.putExtra("message_back", "From Second Activity");
                setResult(RESULT_OK, intent);
                finish();
            }
        });

//        if (getIntent().getStringExtra("guess") != null) {
//            Log.d("Stuff", " " + getIntent().getStringExtra("name"));
//            showGuessTextView.setText(getIntent().getStringExtra("guess"));
//        }

    }

}
