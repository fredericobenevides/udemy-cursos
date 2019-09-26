import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import ColorCounter from '../components/ColorCounter';

const SquareScreen = () => {
  return (
    <View>
      <ColorCounter color="Red"></ColorCounter>
      <ColorCounter color="Blue"></ColorCounter>
      <ColorCounter color="Green"></ColorCounter>
    </View>
  );
};

const styles = StyleSheet.create({});

export default SquareScreen;
