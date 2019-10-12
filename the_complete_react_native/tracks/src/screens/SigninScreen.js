import React, { useContext } from 'react';
import { View, StyleSheet, Text } from 'react-native';
import { NavigationEvents } from 'react-navigation';
import AuthForm from '../components/AuthForm';
import NavLink from '../components/NavLink';
import { Context as context } from '../context/AuthContext';

const SigninScreen = () => {
  const { state, signin, clearErrorMessage } = useContext(context);

  return (
    <View style={styles.container}>
      <NavigationEvents onWillBlur={clearErrorMessage}></NavigationEvents>
      <AuthForm
        headerText="Sign In to Your Account"
        errorMessage={state.errorMessage}
        onSubmit={signin}
        submitButtonText="Sign In"
      ></AuthForm>
      <NavLink
        text="Don't have an account? Sign up instead"
        routeName="Signup"
      ></NavLink>
    </View>
  );
};

SigninScreen.navigationOptions = () => {
  header: null;
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    marginBottom: 250
  }
});

export default SigninScreen;
