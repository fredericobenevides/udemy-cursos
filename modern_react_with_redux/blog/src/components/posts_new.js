import React, { Component } from 'react';
import { reduxForm } from 'redux-form';
import { createPosts } from '../actions/index';
// import { bindActionCreators } from 'redux';

class PostsNew extends Component {
  render() {
   // const title = this.props.fields.title
    const { fields: { title, categories, content }, handleSubmit } = this.props;
    // console.log(title);
    console.log('aqui');
    console.log(this.props);

    return (
      <form onSubmit={handleSubmit(this.props.createPosts)}>
        <h3>Create A New Post</h3>
        <div className="form-group">
          <label>Title</label>
          <input type="text" className="form-control" {...title} />
        </div>

        <div className="form-group">
          <label>Categories</label>
          <input type="text" className="form-control" {...categories} />
        </div>

        <div className="form-group">
          <label>Content</label>
          <textarea className="form-control" {...content} />
        </div>

        <button type="submit" className="btn btn-primary">Submit</button>
      </form>
    );
  }
}

// function mapDispatchToProps(dispatch) {
//   return bindActionCreators({ createPost }, dispatch);
// }

// connect: first argument is mapStateToProps, 2nd is mapDispatchToProps
// reduxForm: first argument is form config, 2nd is mapStateToProps, 3rd is mapDispatchToProps
export default reduxForm({
  form: 'PostsNewForm',
  fields: ['title', 'categories', 'content']
}, null, { createPosts })(PostsNew);
