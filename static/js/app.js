$(document).ready(function(){

  $.get('http://rustpe.org/events', function(data) {
    $.each(data, function(k, v) {
      $('#events').append(v.name, convertDate(v.date), v.about, v.slide);
    })
  });

  function eventTemplate(name, date, about, slide) {
    return '<h3>'+v.name+'</h3>' +
           '<h4>'+convertDate(v.date)+'</h4>' +
           '<p>'+v.about+'</p>' +
           '<p><a href="'+slide+'" target="_blank">Presentación</a>';
  }

  function convertDate(date) {
    return moment(date).format('DD/MM/YYYY hh:mm A');
  }

});
