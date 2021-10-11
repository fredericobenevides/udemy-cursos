import 'package:flutter/material.dart';

void main() {
  runApp(
    MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text("I Am Poor"),
          backgroundColor: Colors.blueGrey,
        ),
        body: Row(
          children: [
            Expanded(
              child: Center(
                child: Image(
                  image: AssetImage('images/hands.jpg'),
                  width: 200,
                  height: 200,
                ),
              ),
            ),
          ],
        ),
      ),
    ),
  );
}
