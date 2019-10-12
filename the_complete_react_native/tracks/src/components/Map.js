import React, { useContext } from 'react';
import { Text, StyleSheet, ActivityIndicator } from 'react-native';
import MapView, { Polyline, Circle } from 'react-native-maps';
import { Context as LocationContext } from '../context/LocationContext';

const Map = () => {
  // let points = [];
  // for (let i = 0; i < 20; i++) {
  //   if (i % 2 === 0) {
  //     points.push({
  //       latitude: 37.33233 + i * 0.001,
  //       longitude: -122.03121 + i * 0.001
  //     });
  //   } else {
  //     points.push({
  //       latitude: 37.33233 - i * 0.002,
  //       longitude: -122.03121 + i * 0.001
  //     });
  //   }
  // }

  const {
    state: { currentLocation, locations }
  } = useContext(LocationContext);
  // console.log(state);

  if (!currentLocation) {
    return (
      <ActivityIndicator
        size="large"
        style={{ marginTop: 200 }}
      ></ActivityIndicator>
    );
  }

  return (
    <MapView
      style={styles.map}
      initialRegion={{
        ...currentLocation.coords,
        // latitude: 37.33233,
        // longitude: -122.03121,
        latitudeDelta: 0.01,
        longitudeDelta: 0.01
      }}
      region={{
        ...currentLocation.coords,
        latitudeDelta: 0.01,
        longitudeDelta: 0.01
      }}
    >
      {/* <Polyline coordinates={points}></Polyline> */}
      <Circle
        center={currentLocation.coords}
        radius={30}
        strokeColor="rgba(158,158,255,1.0)"
        fillColor="rgba(158,158,255,0.3)"
      ></Circle>
      <Polyline coordinates={locations.map(loc => loc.coords)} />
    </MapView>
  );
};

const styles = StyleSheet.create({
  map: {
    height: 300
  }
});

export default Map;
