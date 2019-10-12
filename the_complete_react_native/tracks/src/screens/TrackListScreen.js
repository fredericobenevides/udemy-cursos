import React, { useContext } from 'react';
import {
  View,
  StyleSheet,
  Text,
  Button,
  FlatList,
  TouchableOpacity
} from 'react-native';
import { NavigationEvents } from 'react-navigation';
import { ListItem } from 'react-native-elements';
import { Context as TrackContext } from '../context/TrackContext';

const TrackListScreen = ({ navigation }) => {
  const { state, fetchTracks } = useContext(TrackContext);

  return (
    <>
      <NavigationEvents onWillFocus={fetchTracks} />
      <Text style={{ fontSize: 48 }}>Track List Screen</Text>
      <FlatList
        data={state}
        keyExtractor={item => item._id}
        render={({ item }) => {
          return (
            <TouchableOpacity
              onPress={() => {
                navigation.navigate('TrackDetail', { _id: item._id });
              }}
            >
              <ListItem chevron title={item.name}></ListItem>
            </TouchableOpacity>
          );
        }}
      />
      {/* <Button */}
      {/*   title="Go to Track Detail" */}
      {/*   onPress={() => navigation.navigate('TrackDetail')} */}
      {/* ></Button> */}
    </>
  );
};

TrackListScreen.navigationOptions = {
  title: 'Tracks'
};

const styles = StyleSheet.create({});

export default TrackListScreen;
