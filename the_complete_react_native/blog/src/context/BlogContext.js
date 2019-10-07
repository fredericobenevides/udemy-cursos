import createDataContext from './createDataContext';
import jsonServer from '../api/jsonServer';

const blogReducer = (state, action) => {
  switch (action.type) {
    case 'get_blogposts':
      return action.payload;

    case 'delete_blogpost':
      return state.filter(blogPost => blogPost.id !== action.payload);

    case 'add_blogpost':
      return [
        ...state,
        {
          id: Math.floor(Math.random() * 9999),
          // title: `Blog Post #${state.length + 1}`
          title: action.payload.title,
          content: action.payload.content
        }
      ];

    case 'edit_blogpost':
      return state.map(blogPost => {
        if (blogPost.id === action.payload.id) {
          return action.payload;
        } else {
          return blogPost;
        }
      });

    default:
      return state;
  }
};

const getBlogPosts = dispatch => {
  return async () => {
    const response = await jsonServer.get('/blogPosts');
    dispatch({ type: 'get_blogposts', payload: response.data });
  };
};

const addBlogPost = dispatch => {
  return async (title, content, callback) => {
    await jsonServer.post('/blogposts', { title, content });

    callback && callback();
  };
  // return (title, content, callback) => {
  //   dispatch({ type: 'add_blogpost', payload: { title, content } });
  //   callback && callback();
  // };
};

const deleteBlogPost = dispatch => {
  return async id => {
    console.log('id');
    console.log(id);
    await jsonServer.delete(`/blogposts/${id}`);
    dispatch({ type: 'delete_blogpost', payload: id });
  };
  // return id => {
  //   dispatch({ type: 'delete_blogpost', payload: id });
  // };
};

const editBlogPost = dispatch => {
  return async (id, title, content, callback) => {
    await jsonServer.put(`/blogPosts/${id}`, { title, content });

    dispatch({ type: 'edit_blogpost', payload: { id, title, content } });
    callback && callback();
  };
  // return (id, title, content, callback) => {
  //   dispatch({ type: 'edit_blogpost', payload: { id, title, content } });
  //   callback && callback();
  // };
};

export const { Context, Provider } = createDataContext(
  blogReducer,
  { addBlogPost, deleteBlogPost, editBlogPost, getBlogPosts },
  [] // new initial data, since is getting from json-server
  // [{ title: 'TEST POST', content: 'TEST CONTENT', id: 1 }] // old initial
  // data
);
