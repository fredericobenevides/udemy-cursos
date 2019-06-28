package com.bawp.trivia.data;

import android.util.Log;

import com.android.volley.Request;
import com.android.volley.Response;
import com.android.volley.VolleyError;
import com.android.volley.toolbox.JsonArrayRequest;
import com.bawp.trivia.controller.AppController;
import com.bawp.trivia.model.Question;

import org.json.JSONArray;
import org.json.JSONException;

import java.util.ArrayList;
import java.util.List;

public class QuestionBank {

    ArrayList<Question> questionArrayList = new ArrayList<Question>();

    private String url = "https://raw.githubusercontent.com/curiousily/simple-quiz/master/script/statements-data.json";

    public List<Question> getQuestions(final AnswerListAsyncResponse callBack) {
        JsonArrayRequest jsonArrayRequest = new JsonArrayRequest(
                Request.Method.GET,
                url,
                (JSONArray) null,
                new Response.Listener<JSONArray>() {
                    @Override
                    public void onResponse(JSONArray response) {
                        for (int i = 0; i < response.length(); i++) {
                            try {
                                String answer = response.getJSONArray(i).get(0).toString();
                                boolean answerTrue = response.getJSONArray(i).getBoolean(1);

                                Question question = new Question();
                                question.setAnswer(answer);
                                question.setAnswerTrue(answerTrue);

                                // Add question objects to list
                                questionArrayList.add(question);

//                                Log.d("JSON Stuff", "onResponse: " + answer);
//                                Log.d("JSON Stuff", "onResponse: " + answerTrue);
                            } catch (JSONException e) {
                                e.printStackTrace();
                            }
                        }

                        if (null != callBack) {
                            callBack.processFinished(questionArrayList);
                        }
                    }
                }, new Response.ErrorListener() {
            @Override
            public void onErrorResponse(VolleyError error) {

            }
        });

        AppController.getInstance().addToRequestQueue(jsonArrayRequest);

        return questionArrayList;
    }
}
