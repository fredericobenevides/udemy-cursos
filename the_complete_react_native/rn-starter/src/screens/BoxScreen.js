import React, { useState } from 'react';
import { View, Text, StyleSheet } from 'react-native';

const BoxScreen = () => {
  return (
    <View style={styles.viewParentStyle}>
      {/* <Text style={styles.textOneStyle}>Child #1</Text> */}
      {/* <Text style={styles.textTwoStyle}>Child #2</Text> */}
      {/* <Text style={styles.textThreeStyle}>Child #3</Text> */}
      <View style={styles.viewOneStyle}></View>
      <View style={styles.viewTwoStyle}></View>
      <View style={styles.viewThreeStyle}></View>
    </View>
  );
};

const styles = StyleSheet.create({
  viewParentStyle: {
    borderWidth: 3,
    borderColor: 'black',
    height: 200,
    flexDirection: 'row',
    justifyContent: 'space-between'
  },
  viewOneStyle: {
    height: 50,
    width: 50,
    backgroundColor: 'red'
  },
  viewTwoStyle: {
    height: 50,
    width: 50,
    backgroundColor: 'green',
    // marginTop: 50
    // alignSelf: 'flex-end',
    top: 50
  },
  viewThreeStyle: {
    height: 50,
    width: 50,
    backgroundColor: 'purple'
  }

  // viewStyle: {
  //   borderWidth: 3,
  //   borderColor: 'black',
  //   height: 200
  //   // flexDirection: 'row'
  //   // alignItems: 'center'
  //   // justifyContent: 'flex-start'
  // },
  // textOneStyle: {
  //   borderWidth: 3,
  //   borderColor: 'red'
  // },
  // textTwoStyle: {
  //   borderWidth: 3,
  //   borderColor: 'red',
  //   ...StyleSheet.absoluteFillObject
  // },
  // textThreeStyle: {
  //   borderWidth: 3,
  //   borderColor: 'red'
  // }
});

export default BoxScreen;
