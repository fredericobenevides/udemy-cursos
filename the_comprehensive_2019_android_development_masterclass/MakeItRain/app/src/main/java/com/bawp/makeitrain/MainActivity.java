package com.bawp.makeitrain;

import androidx.appcompat.app.AppCompatActivity;

import android.graphics.Color;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

import java.text.NumberFormat;

public class MainActivity extends AppCompatActivity {

//    private Button showMoney;
//    private Button showTag;

    private TextView moneyText;
    private int moneyCounter = 0;

    private Button newButton;


    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.another_layout);

        moneyText = findViewById(R.id.money_text);

        newButton = findViewById(R.id.button);
        newButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                Log.d("New button", "onClick: Hola");
            }
        });

//        showMoney = findViewById(R.id.button_make_rain);
//        showTag = findViewById(R.id.button_show_tag);

//        showMoney.setOnClickListener(new View.OnClickListener() {
//            @Override
//            public void onClick(View view) {
//                Log.d("MYTAG", "onClick: Show Money");
//            }
//        });
    }

    public void showTag(View view) {
        Toast.makeText(getApplicationContext(), R.string.app_name, Toast.LENGTH_SHORT).show();
//        Log.d("MYTAG", "onClick: Show Tag");
    }

    public void makeItRain(View view) {
        NumberFormat numberFormat = NumberFormat.getCurrencyInstance();

        moneyCounter += 1000;
        moneyText.setText(String.valueOf(numberFormat.format(moneyCounter)));

//        moneyText.setTextColor(getResources().getColor(R.color.myColor));

        switch (moneyCounter) {
            case 20000:
                moneyText.setTextColor(Color.BLACK);

                break;
            case 40000:
                moneyText.setTextColor(Color.YELLOW);
                break;
            case 60000:
                moneyText.setTextColor(Color.GREEN);
                break;
            default:
                break;
        }

//        Log.d("MYTAG", "makeItRain: Tapped " + moneyCounter);
    }
}
