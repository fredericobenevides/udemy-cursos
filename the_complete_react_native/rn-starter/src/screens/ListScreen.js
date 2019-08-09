import React from 'react';
import { View, Text, StyleSheet, FlatList } from 'react-native';

const ListScreen = () => {
  const friends = [
    { name: 'Fried #1' },
    { name: 'Fried #2' },
    { name: 'Fried #3' },
    { name: 'Fried #4' },
    { name: 'Fried #5' },
    { name: 'Fried #6' },
    { name: 'Fried #7' },
    { name: 'Fried #8' },
    { name: 'Fried #9' }
  ];

  return (
    <FlatList
      data={friends}
      // renderItem={({ element }) => {
      renderItem={({ item }) => {
        // element === { item: { name: 'Friend #1' }, index: 0 }
        return <Text>{item.name}</Text>;
      }}
    ></FlatList>
  );
};

const styles = StyleSheet.create({});

export default ListScreen;
