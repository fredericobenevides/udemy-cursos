import React, { useState } from 'react';
import { View, Text, StyleSheet, Button } from 'react-native';

const CounterScreen = () => {
  const [counter, setCounter] = useState(0);

  return (
    <View>
      <Button
        title="Increase"
        onPress={() => {
          // Dont do this
          // counter++;
          setCounter(counter + 1);
          // console.log(counter);
        }}
      ></Button>

      <Button
        title="Decrease"
        onPress={() => {
          // counter--;
          // console.log(counter);
          setCounter(counter - 1);
        }}
      ></Button>

      <Text>Current Count: {counter}</Text>
    </View>
  );
};

const styles = StyleSheet.create({});

export default CounterScreen;
