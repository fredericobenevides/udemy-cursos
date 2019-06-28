package com.bawp.contactmanager;

import androidx.appcompat.app.AppCompatActivity;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.AdapterView;
import android.widget.ArrayAdapter;
import android.widget.ListView;

import com.bawp.contactmanager.adapter.RecyclerViewAdapter;
import com.bawp.contactmanager.data.DatabaseHandler;
import com.bawp.contactmanager.model.Contact;

import java.util.ArrayList;
import java.util.List;

public class MainActivity extends AppCompatActivity {

    private RecyclerView recyclerView;
    private RecyclerViewAdapter recyclerViewAdapter;

//    private ListView listView;
    private ArrayList<Contact> contactArrayList;
    private ArrayAdapter<String> arrayAdapter;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        recyclerView = findViewById(R.id.recylerView);
        recyclerView.setHasFixedSize(true);
        recyclerView.setLayoutManager(new LinearLayoutManager(this));


        DatabaseHandler db = new DatabaseHandler(MainActivity.this);

        contactArrayList = new ArrayList<>();
        final List<Contact> contactList = db.getAllContacts();
        for (Contact contact : contactList) {
            Log.d("MainActivity", "onCreate: " + contact.getName());

            contactArrayList.add(contact);
        }

        // setup adapter
        recyclerViewAdapter = new RecyclerViewAdapter(MainActivity.this, contactArrayList);
        recyclerView.setAdapter(recyclerViewAdapter);

//        listView = findViewById(R.id.listview);



//        Contact a = new Contact("Greg", "91231");
//        db.addContact(a);
//
//        db.addContact(new Contact("James", "123132"));
//        db.addContact(new Contact("Greg", "4545"));
//        db.addContact(new Contact("Helena", "34242"));
//        db.addContact(new Contact("Carimo", "756573"));
//        db.addContact(new Contact("Silo", "543231"));
//        db.addContact(new Contact("Santos", "3242"));
//        db.addContact(new Contact("Litos", "1213"));
//        db.addContact(new Contact("Karate", "45352"));


        /*
        Using list view
         */

//        // create our array adapter
//        arrayAdapter = new ArrayAdapter<>(
//                this,
//                android.R.layout.simple_list_item_1,
//                contactArrayList
//        );
//
//        // add to our listview
//        listView.setAdapter(arrayAdapter);
//
//        // Atttach eventlistener to listview
//        listView.setOnItemClickListener(new AdapterView.OnItemClickListener() {
//            @Override
//            public void onItemClick(AdapterView<?> adapterView, View view, int position, long id) {
//                Log.d("List", "onItemClick: " + contactArrayList.get(position));
//            }
//        });
    }
}
