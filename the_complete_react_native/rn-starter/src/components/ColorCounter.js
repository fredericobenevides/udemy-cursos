import React from 'react';
import { View, Text, StyleSheet, Button } from 'react-native';

const ColorCounter = ({ color, onIncrease, onDecrease }) => {
  console.log(color);
  console.log(onIncrease);
  return (
    <View>
      <Text>More {color}</Text>
      <Button onPress={() => onIncrease} title={`Increase ${color}`}></Button>
      <Button onPress={() => onDecrease} title={`Decrease ${color}`}></Button>
    </View>
  );
};

const styles = StyleSheet.create({});

export default ColorCounter;
