import React, { useContext } from 'react';
// import { View, StyleSheet, Text, Button } from 'react-native';
import { View, StyleSheet } from 'react-native';
import { NavigationEvents } from 'react-navigation';
import { Context as AuthContext } from '../context/AuthContext';
import AuthForm from '../components/AuthForm';
import NavLink from '../components/NavLink';

const SignupScreen = ({ navigation }) => {
  const { state, signup, clearErrorMessage } = useContext(AuthContext);

  console.log(state);

  return (
    <View style={styles.container}>
      <NavigationEvents onWillBlur={clearErrorMessage}></NavigationEvents>
      <AuthForm
        headerText="Sign Up for Tracker"
        errorMessage={state.errorMessage}
        submitButtonText="Sign Up"
        onSubmit={signup}
        // onSubmit={({ email, password }) => signup({ email, password })}
      ></AuthForm>
      <NavLink
        routeName="Signin"
        text="Already have an account? Sign in instead!"
      ></NavLink>
    </View>
  );
};

SignupScreen.navigationOptions = () => {
  return {
    header: null
  };
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    marginBottom: 200
  }
});

export default SignupScreen;
