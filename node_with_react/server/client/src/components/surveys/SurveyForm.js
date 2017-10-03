import _ from 'lodash';
import React, { Component } from 'react';
import { reduxForm, Field } from 'redux-form';
import { Link } from 'react-router-dom';
import SurveyField from './SurveyField';
import validateEmails from '../../utils/validateEmails';
import formFields from './formFields';

// const FIELDS = [
//   { label: 'Survey Title', name: 'title' },
//   { label: 'Subject Line', name: 'subject' },
//   { label: 'Email Body', name: 'body' },
//   { label: 'Recipient List', name: 'emails' }
// ];

class SurveyForm extends Component {
  renderFields() {
    return _.map(formFields, ({ label, name }) => {
      return (
        <Field
          key={name}
          component={SurveyField}
          type="text"
          label={label}
          name={name}
        />
      );
    });
  }

  render() {
    return (
      <div>
        <form onSubmit={this.props.handleSubmit(this.props.onSurveySubmit)}>
          {this.renderFields()}

          <Link to="/surveys" className="red btn-flat white-text">
            Cancel
          </Link>

          <button type="submit" className="teal btn-flat right white-text">
            Next
            <i className="material-icons right">done</i>
          </button>
        </form>
      </div>
    );
  }
}

function validate(values) {
  const errors = {};

  _.each(formFields, ({ name }) => {
    if (!values[name]) {
      errors[name] = 'You must a provide a value';
    }
  });

  errors.recipients = validateEmails(values.recipients || '');

  // if (!values.title) {
  //   errors.title = 'You must provide a title';
  // }
  //
  // if (!values.subject) {
  //   errors.title = 'You must provide a subject';
  // }
  //
  // if (!values.title) {
  //   errors.title = 'You must provide a body';
  // }

  return errors;
}

// export default SurveyForm;
export default reduxForm({
  // validate: validate
  validate,
  form: 'surveyForm',
  destroyOnUnmount: false
})(SurveyForm);
