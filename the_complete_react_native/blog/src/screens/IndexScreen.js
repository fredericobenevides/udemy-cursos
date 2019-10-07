import React, { useContext, useEffect } from 'react';
import {
  View,
  Text,
  StyleSheet,
  FlatList,
  Button,
  TouchableOpacity
} from 'react-native';
import { Context as BlogContext } from '../context/BlogContext';
import { Feather } from '@expo/vector-icons';

const IndexScreen = ({ navigation }) => {
  const { state, addBlogPost, deleteBlogPost, getBlogPosts } = useContext(
    BlogContext
  );

  useEffect(() => {
    // getBlogPosts();

    // veio de outra janela ou teve focus, carrega todos os posts
    const listener = navigation.addListener('didFocus', () => {
      getBlogPosts();
    });

    // quando sair da tela, o use effect irá chamar esse callback e removerá
    // o listener
    return () => {
      listener.remove();
    };
  }, []);

  return (
    <View>
      {/* <Text>Index Screen</Text> */}
      {/* <Button title="Add Post" onPress={addBlogPost}></Button> */}
      <FlatList
        data={state}
        keyExtractor={blogPost => blogPost.title}
        renderItem={({ item }) => {
          return (
            <TouchableOpacity
              onPress={() => navigation.navigate('Show', { id: item.id })}
            >
              <View style={styles.row}>
                <Text style={styles.title}>
                  {item.title} - {item.id}
                </Text>

                <TouchableOpacity onPress={() => deleteBlogPost(item.id)}>
                  <Feather style={styles.icon} name="trash"></Feather>
                </TouchableOpacity>
              </View>
            </TouchableOpacity>
          );
        }}
      ></FlatList>
    </View>
  );
};

IndexScreen.navigationOptions = ({ navigation }) => {
  return {
    headerRight: (
      <TouchableOpacity onPress={() => navigation.navigate('Create')}>
        <Feather name="plus" size={30}></Feather>
      </TouchableOpacity>
    )
  };
};

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: 20,
    paddingHorizontal: 10,
    borderBottomWidth: 1,
    borderColor: 'gray'
  },
  title: {
    fontSize: 18
  },
  icon: {
    fontSize: 24
  }
});

export default IndexScreen;
